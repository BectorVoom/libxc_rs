//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1306/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1306<F: Float>(t11806: F, t21943: F, t7923: F, t1394: F, t21057: F, t27387: F, t2109: F, t27614: F, t6176: F, t6183: F, t101875: F, t102061: F, t102328: F, t27607: F, t28714: F, t28721: F, t28727: F, t28738: F, t28811: F, t28816: F, t29526: F, t7968: F, t7978: F, t94861: F) -> (F, F, F, F) {
    let t102348 = t11806 * t7923 * t21943;
    let t102357 = t1394 * t27387 * t21057;
    let t102371 = t6176 * t27614 * t6183 * t2109;
    let t102374 = F::new(0.51588271604938271605e-2) * t102348 + F::new(0.18534722222222222222e-2) * t28727 * t28816 + F::new(0.18534722222222222222e-2) * t28727 * t28738 - F::new(0.2782641015625e-3) * t7968 * t102061 - F::new(0.11607361111111111111e-2) * t102357 - F::new(0.13901041666666666667e-2) * t28714 * t28811 + F::new(0.13913205078125e-3) * t7968 * t102328 - F::new(0.92754700520833333334e-4) * t28721 * t28816 + F::new(0.24777891269883300782e-5) * t94861 * t101875 - F::new(0.69505208333333333334e-3) * t27607 * t29526 - F::new(0.69505208333333333334e-3) * t7978 * t102371;
    (t102348, t102357, t102371, t102374)
}
