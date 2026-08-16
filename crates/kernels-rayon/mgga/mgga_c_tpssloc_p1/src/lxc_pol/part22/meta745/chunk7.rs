//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2480/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2480(t10403: f64, t10422: f64, t21525: f64, t1023: f64, t10408: f64, t1041: f64, t10876: f64, t14508: f64, t1539: f64, t17670: f64, t17714: f64, t17732: f64, t17890: f64, t17960: f64, t21118: f64, t21398: f64, t21512: f64, t3048: f64, t3070: f64, t3071: f64, t42565: f64, t4582: f64, t4644: f64, t47779: f64, t62210: f64, t62234: f64, t70330: f64) -> f64 {
    let t70535 = t10403 * t10422 * t21525;
    let t70539 = t42565 * t21398 / 96.0_f64 + 5.0_f64 / 2592.0_f64 * t62210 - 3.0_f64 / 512.0_f64 * t10876 * t4582 * t17670 * t17732 + t3070 * t3071 * t17960 * t1539 / 1536.0_f64 - t62234 / 1152.0_f64 + 5.0_f64 / 384.0_f64 * t1041 * t4582 * t47779 * t70330 - 5.0_f64 / 2304.0_f64 * t3070 * t10408 * t21118 * t1023 + t14508 * t17714 / 512.0_f64 - 5.0_f64 / 864.0_f64 * t3048 * t21512 + t70535 / 1152.0_f64 + t4644 * t17890 / 1536.0_f64;
    t70539
}
