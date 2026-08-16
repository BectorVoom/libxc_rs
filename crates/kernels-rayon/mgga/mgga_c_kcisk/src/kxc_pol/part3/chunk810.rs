//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 810/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk810(t12467: f64, t3: f64, t213: f64, t12476: f64, t2862: f64, t12485: f64, t817: f64, t5680: f64, t12468: f64, t12469: f64, t12473: f64, t12480: f64, t15: f64, t2863: f64, t2866: f64, t3092: f64, t3096: f64, t818: f64, t947: f64) -> (f64, f64, f64, f64) {
    let t12488 = t12467 * t3;
    let t12489 = t12488 * t213;
    let t12491 = t2862 * t12476;
    let t12493 = t817 * t12485;
    let t12496 = -0.26426666666666666667e-1_f64 * t12489 + 0.17617777777777777778e-1_f64 * t12491 - 0.20554074074074074074e-1_f64 * t12493 - 0.12841111111111111111e-1_f64 * t5680;
    let t12499 = -t12468 * t12469 / 3.0_f64 - t12473 * t2863 / 6.0_f64 + 2.0_f64 / 9.0_f64 * t3092 * t12476 - t12480 * t818 / 4.0_f64 + t3096 * t2866 / 3.0_f64 - 7.0_f64 / 27.0_f64 * t947 * t12485 + t15 * t12496 / 2.0_f64;
    (t12489, t12491, t12493, t12499)
}
