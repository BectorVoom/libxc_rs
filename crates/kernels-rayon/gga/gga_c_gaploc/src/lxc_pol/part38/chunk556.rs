//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 556/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk556(t1: f64, t3338: f64, t106: f64, t192: f64, t3377: f64, t8155: f64, t8158: f64, t10465: f64, t10468: f64, t10472: f64, t10476: f64, t10479: f64, t10480: f64, t10484: f64, t10485: f64, t10489: f64, t10492: f64, t1456: f64, t2386: f64, t536: f64, t574: f64, t597: f64, t9540: f64, t9546: f64) -> (f64, f64, f64) {
    let t10495 = t3338 * t1;
    let t10496 = t10495 * t106;
    let t10497 = t10496 * t192;
    let t10501 = 0.10725146985555128001e1_f64 * t8155 * t3377;
    let t10503 = 0.10725146985555128001e1_f64 * t8158 * t3377;
    let t10504 = t10465 - t10468 - t10472 + t10476 - t9540 + t9546 + t10479 + 0.35750489951850426669e0_f64 * t1456 * t10480 + t10484 - 0.10725146985555128001e1_f64 * t10485 * t2386 - 0.46011511144704899612e1_f64 * t574 * t10489 + 0.11502877786176224903e2_f64 * t597 * t10492 + 0.35750489951850426669e0_f64 * t536 * t10497 - t10501 - t10503;
    (t10496, t10497, t10504)
}
