//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1306/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1306(t11806: f64, t21943: f64, t7923: f64, t1394: f64, t21057: f64, t27387: f64, t2109: f64, t27614: f64, t6176: f64, t6183: f64, t101875: f64, t102061: f64, t102328: f64, t27607: f64, t28714: f64, t28721: f64, t28727: f64, t28738: f64, t28811: f64, t28816: f64, t29526: f64, t7968: f64, t7978: f64, t94861: f64) -> (f64, f64, f64, f64) {
    let t102348 = t11806 * t7923 * t21943;
    let t102357 = t1394 * t27387 * t21057;
    let t102371 = t6176 * t27614 * t6183 * t2109;
    let t102374 = 0.51588271604938271605e-2_f64 * t102348 + 0.18534722222222222222e-2_f64 * t28727 * t28816 + 0.18534722222222222222e-2_f64 * t28727 * t28738 - 0.2782641015625e-3_f64 * t7968 * t102061 - 0.11607361111111111111e-2_f64 * t102357 - 0.13901041666666666667e-2_f64 * t28714 * t28811 + 0.13913205078125e-3_f64 * t7968 * t102328 - 0.92754700520833333334e-4_f64 * t28721 * t28816 + 0.24777891269883300782e-5_f64 * t94861 * t101875 - 0.69505208333333333334e-3_f64 * t27607 * t29526 - 0.69505208333333333334e-3_f64 * t7978 * t102371;
    (t102348, t102357, t102371, t102374)
}
