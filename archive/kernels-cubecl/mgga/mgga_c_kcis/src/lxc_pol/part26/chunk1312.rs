//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1312/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1312<F: Float>(t1459: F, t303: F, t7203: F, t29600: F, t7974: F, t102464: F, t102467: F, t102475: F, t102478: F, t102481: F, t28749: F, t28755: F, t95088: F, t98573: F, t99301: F) -> (F, F) {
    let t102484 = t303 * t1459 * t7203;
    let t102486 = t29600 * t7974;
    let t102488 = t95088 - F::cast_from(0.17411041666666666666e-2_f64) * t102464 + F::cast_from(0.34822083333333333332e-2_f64) * t102467 + F::cast_from(0.61905925925925925925e-2_f64) * t98573 + F::cast_from(0.23168402777777777778e-3_f64) * t99301 * t28749 + F::cast_from(0.23168402777777777778e-3_f64) * t99301 * t28755 - F::cast_from(0.17024129629629629629e-1_f64) * t102475 + F::cast_from(0.11349419753086419753e-1_f64) * t102478 - F::cast_from(0.61905925925925925925e-2_f64) * t102481 + F::cast_from(0.11607361111111111111e-2_f64) * t102484 - F::cast_from(0.11326774691358024691e-2_f64) * t102486;
    (t102484, t102488)
}
