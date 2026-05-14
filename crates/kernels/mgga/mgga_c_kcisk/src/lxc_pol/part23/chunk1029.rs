//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1029/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1029<F: Float>(t1375: F, t19127: F, t1398: F, t19136: F, t13987: F, t13989: F, t13991: F, t13993: F, t14001: F, t14003: F, t14011: F, t14014: F, t14025: F, t14027: F, t14029: F, t14031: F, t14056: F, t158: F, t173: F, t20657: F, t20660: F, t5808: F, t5823: F) -> (F,) {
    let t20662 = t1375 * t19127;
    let t20665 = t1398 * t19136;
    let t20668 = -0.9368e-2 * t13987 - 0.21858666666666666666e-1 * t13989 - 0.117630625e-4 * t13991 + 0.15684083333333333333e-4 * t13993 + 0.4684e-2 * t14001 - 0.15613333333333333333e-2 * t14003 - 0.13208333333333333333e-2 * t14011 + 0.88055555555555555553e-3 * t14014 + 0.26416666666666666666e-2 * t14025 + 0.70444444444444444443e-2 * t14027 + 0.78420416666666666666e-4 * t14029 + 0.23526125e-4 * t14031 - 0.10359077815592613752e-3 * t5808 - 0.26887e-4 * t5823 * t20657 + 0.31368166666666666666e-4 * t20660 - 0.21078e-1 * t158 * t20662 - 0.10082625e-4 * t173 * t20665 + t14056;
    (t20668,)
}
