//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1250/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1250<F: Float>(t26246: F, t8392: F, t10961: F, t22943: F, t12001: F, t26330: F, t102045: F, t446: F, t83: F, t91876: F, t91881: F, t91883: F, t91895: F, t91897: F, t91899: F, t91901: F, t91903: F, t91905: F, t91912: F, t91926: F) -> (F, F) {
    let t103343 = 2.0 / 27.0 * t8392 * t26246;
    let t103346 = t22943 * t10961;
    let t103350 = t12001 * t26330;
    let t103363 = -t91876 / 9.0 - t103343 + 2.0 / 27.0 * t91881 - t91883 / 27.0 + 4.0 / 3.0 * t446 * t83 * t103346 + 22.0 / 27.0 * t103350 - 2.0 * t446 * t83 * t102045 - 8.0 / 81.0 * t91895 + 8.0 / 27.0 * t91897 + 2.0 / 9.0 * t91899 + 2.0 / 3.0 * t91901 - 2.0 / 9.0 * t91903 - 4.0 / 9.0 * t91905 - 4.0 / 9.0 * t91912 + 2.0 / 27.0 * t91926;
    (t103346, t103363)
}
