//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1313/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1313(t102190: f64, t7968: f64, t1464: f64, t1489: f64, t27387: f64, t7313: f64, t18210: f64, t29568: f64, t7978: f64, t28727: f64, t28778: f64, t28714: f64, t28721: f64, t28772: f64, t28835: f64, t8226: f64, t99013: f64, t99476: f64, t99478: f64, t99480: f64, t99494: f64) -> (f64, f64) {
    let t102494 = t7968 * t102190;
    let t102498 = t1464 * t27387 * t7313 * t1489;
    let t102503 = t7978 * t18210 * t29568;
    let t102507 = t28727 * t28778;
    let t102509 = -0.41188271604938271605e-3_f64 * t99476 - t99478 - t99480 + 0.69505208333333333334e-3_f64 * t99013 * t8226 + 0.69505208333333333334e-3_f64 * t28714 * t28835 + 0.15459116753472222222e-4_f64 * t102494 - 0.11607361111111111111e-2_f64 * t102498 + 0.92754700520833333334e-4_f64 * t28721 * t28772 + t99494 - 0.23168402777777777778e-3_f64 * t102503 - 0.18534722222222222222e-2_f64 * t28727 * t28772 - 0.61782407407407407407e-3_f64 * t102507;
    (t102498, t102509)
}
