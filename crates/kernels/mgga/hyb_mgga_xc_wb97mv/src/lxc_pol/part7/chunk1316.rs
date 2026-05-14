//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1316/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1316<F: Float>(t27021: F, t27024: F, t27027: F, t27207: F, t27210: F, t27213: F, t31779: F, t31782: F, t31810: F, t31860: F, t31869: F, t31873: F, t31880: F, t31883: F, t31886: F, t31889: F, t31891: F, t31893: F, t31896: F, t31898: F, t31900: F, t31902: F, t31905: F, t31907: F) -> (F, F) {
    let t32130 = -0.3529725e1 * t31860 - 0.32136222222222222223e1 * t27021 + 0.27545333333333333334e1 * t27024 - 0.103295e1 * t27027 - 0.41678e0 * t27207 - 0.83356e0 * t27210 - 0.41678e0 * t27213 + 0.34731666666666666667e0 * t31869 + 0.62517e0 * t31873 + 0.68863333333333333333e0 * t31779 - 0.103295e1 * t31782 + 0.1549425e1 * t31810;
    let t32143 = -0.6618234375e1 * t31880 + 0.264729375e1 * t31883 + 0.2366859375e0 * t31886 - 0.157790625e0 * t31889 + 0.264729375e1 * t31891 - 0.3529725e1 * t31893 - 0.3529725e1 * t31896 - 0.17648625e1 * t31898 - 0.157790625e0 * t31900 + 0.6311625e0 * t31902 + 0.6311625e0 * t31905 + 0.31558125e0 * t31907;
    (t32130, t32143)
}
