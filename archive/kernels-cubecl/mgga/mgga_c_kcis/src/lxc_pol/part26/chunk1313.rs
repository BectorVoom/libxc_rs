//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1313/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1313<F: Float>(t102190: F, t7968: F, t1464: F, t1489: F, t27387: F, t7313: F, t18210: F, t29568: F, t7978: F, t28727: F, t28778: F, t28714: F, t28721: F, t28772: F, t28835: F, t8226: F, t99013: F, t99476: F, t99478: F, t99480: F, t99494: F) -> (F, F) {
    let t102494 = t7968 * t102190;
    let t102498 = t1464 * t27387 * t7313 * t1489;
    let t102503 = t7978 * t18210 * t29568;
    let t102507 = t28727 * t28778;
    let t102509 = -F::cast_from(0.41188271604938271605e-3_f64) * t99476 - t99478 - t99480 + F::cast_from(0.69505208333333333334e-3_f64) * t99013 * t8226 + F::cast_from(0.69505208333333333334e-3_f64) * t28714 * t28835 + F::cast_from(0.15459116753472222222e-4_f64) * t102494 - F::cast_from(0.11607361111111111111e-2_f64) * t102498 + F::cast_from(0.92754700520833333334e-4_f64) * t28721 * t28772 + t99494 - F::cast_from(0.23168402777777777778e-3_f64) * t102503 - F::cast_from(0.18534722222222222222e-2_f64) * t28727 * t28772 - F::cast_from(0.61782407407407407407e-3_f64) * t102507;
    (t102498, t102509)
}
