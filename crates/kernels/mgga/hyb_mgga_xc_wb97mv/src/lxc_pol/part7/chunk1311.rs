//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1311/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1311<F: Float>(t27021: F, t27024: F, t27027: F, t27207: F, t27210: F, t27213: F, t31779: F, t31782: F, t31810: F, t31860: F, t31869: F, t31873: F, t31880: F, t31883: F, t31886: F, t31889: F, t31891: F, t31893: F, t31896: F, t31898: F, t31900: F, t31902: F, t31905: F, t31907: F) -> (F, F) {
    let t32020 = -0.258925e1 * t31860 - 0.18786444444444444444e1 * t27021 + 0.16102666666666666667e1 * t27024 - 0.60385e0 * t27027 - 0.33114e0 * t27207 - 0.66228e0 * t27210 - 0.33114e0 * t27213 + 0.27595e0 * t31869 + 0.49671e0 * t31873 + 0.40256666666666666667e0 * t31779 - 0.60385e0 * t31782 + 0.905775e0 * t31810;
    let t32033 = -0.485484375e1 * t31880 + 0.19419375e1 * t31883 + 0.6189328125e-1 * t31886 - 0.412621875e-1 * t31889 + 0.19419375e1 * t31891 - 0.258925e1 * t31893 - 0.258925e1 * t31896 - 0.1294625e1 * t31898 - 0.412621875e-1 * t31900 + 0.16504875e0 * t31902 + 0.16504875e0 * t31905 + 0.82524375e-1 * t31907;
    (t32020, t32033)
}
