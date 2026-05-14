//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 947/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk947<F: Float>(t838: F, t847: F, t9000: F, t8911: F, t6762: F, t6765: F, t6993: F, t8908: F, t8929: F, t6986: F, t284: F, t6817: F, t6820: F, t6823: F, t6884: F, t6891: F, t8952: F, t8955: F, t8958: F) -> (F, F, F, F, F, F, F, F) {
    let t9002 = t838 * t9000 * t847;
    let t9012 = 0.34246666666666666666e-1 * t8911;
    let t9014 = -t6993 + 0.45662222222222222222e-1 * t6762 - 0.17123333333333333333e-1 * t6765 + 0.22831111111111111111e-1 * t8908 - t9012 + 0.5137e-1 * t8929;
    let t9020 = 0.18541666666666666667e-1 * t8911;
    let t9022 = -t6986 + 0.24722222222222222222e-1 * t6762 - 0.92708333333333333333e-2 * t6765 + 0.12361111111111111111e-1 * t8908 - t9020 + 0.278125e-1 * t8929;
    let t9023 = t9022 * t284;
    let t9028 = 0.103295e1 * t8911;
    let t9035 = -t6891 + 0.69463333333333333333e0 * t6817 - 0.20839e0 * t6820 - 0.20839e0 * t6823 - t6884 - t9028 + 0.1549425e1 * t8929 + 0.13772666666666666667e1 * t6762 - 0.516475e0 * t6765 + 0.264729375e1 * t8952 - 0.157790625e0 * t8955 + 0.34731666666666666667e0 * t8958;
    (t9002, t9012, t9014, t9020, t9022, t9023, t9028, t9035)
}
