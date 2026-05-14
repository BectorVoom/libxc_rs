//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1276/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1276<F: Float>(t30832: F, t30835: F, t30837: F, t30840: F, t30843: F, t30846: F, t30849: F, t30851: F, t30853: F, t30856: F, t30858: F, t26298: F, t26301: F, t26304: F, t30747: F, t30750: F, t30778: F, t30861: F, t30867: F, t30871: F, t30877: F, t30879: F, t30882: F) -> (F, F) {
    let t31275 = 0.6311625e0 * t30832 + 0.6311625e0 * t30835 + 0.31558125e0 * t30837 - 0.6618234375e1 * t30840 + 0.264729375e1 * t30843 + 0.2366859375e0 * t30846 - 0.157790625e0 * t30849 + 0.264729375e1 * t30851 - 0.3529725e1 * t30853 - 0.3529725e1 * t30856 - 0.17648625e1 * t30858;
    let t31288 = -0.157790625e0 * t30861 - 0.32136222222222222223e1 * t26298 + 0.27545333333333333334e1 * t26301 - 0.103295e1 * t26304 + 0.34731666666666666667e0 * t30867 + 0.62517e0 * t30871 + 0.68863333333333333333e0 * t30747 - 0.103295e1 * t30750 + 0.1549425e1 * t30778 + 0.10589175e2 * t30877 - 0.6311625e0 * t30879 - 0.3529725e1 * t30882;
    (t31275, t31288)
}
