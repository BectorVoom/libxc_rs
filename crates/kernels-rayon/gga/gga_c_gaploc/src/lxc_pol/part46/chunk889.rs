//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 889/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk889(t2386: f64, t3338: f64, t544: f64, t6514: f64, t40549: f64, t40555: f64, t40558: f64, t40561: f64, t40564: f64, t40567: f64, t40570: f64, t1456: f64, t1457: f64, t40546: f64, t42086: f64, t42421: f64, t42422: f64, t42425: f64, t42429: f64, t42432: f64, t42435: f64, t42438: f64, t42442: f64, t42444: f64, t42448: f64) -> f64 {
    let t42452 = t544 * t6514 * t3338 * t2386;
    let t42455 = 0.11916829983950142223e0_f64 * t40549;
    let t42456 = 0.89376224879626066674e-1_f64 * t40555;
    let t42457 = 0.59584149919750711116e-1_f64 * t40558;
    let t42458 = 0.59584149919750711116e-1_f64 * t40561;
    let t42459 = 0.1022478025437886658e1_f64 * t40564;
    let t42460 = 0.25561950635947166451e1_f64 * t40567;
    let t42461 = 0.29792074959875355558e-1_f64 * t40570;
    let t42462 = -t42421 - 0.38342925953920749676e0_f64 * t42422 - 0.38342925953920749676e0_f64 * t42425 + t42429 - t42432 - 0.15889106645266856298e0_f64 * t42435 + t42438 + t42442 - t42444 + 0.35750489951850426669e0_f64 * t1456 * t1457 * t42086 - 0.14300195980740170668e1_f64 * t42448 - 0.50050685932590597338e1_f64 * t42452 + 0.38342925953920749676e0_f64 * t40546 + t42455 - t42456 + t42457 - t42458 + t42459 - t42460 + t42461;
    t42462
}
