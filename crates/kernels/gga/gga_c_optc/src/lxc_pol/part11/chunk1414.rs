//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1414/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1414<F: Float>(t5122: F, t33724: F, t33730: F, t43414: F, t44193: F, t44198: F, t52389: F, t52391: F, t52393: F, t58348: F, t58352: F, t58356: F, t58360: F, t58363: F, t58367: F) -> (F, F) {
    let t59263 = t5122 * t5122;
    let t59281 = -F::cast_from(0.23154444444444444445e0_f64) * t44193 + F::cast_from(0.13892666666666666667e1_f64) * t44198 - F::cast_from(0.91817777777777777776e0_f64) * t43414 + F::cast_from(0.21424148148148148148e1_f64) * t33724 + F::cast_from(0.12349037037037037037e1_f64) * t33730 + F::new(0.41318e1) * t58348 + F::new(0.125034e1) * t58352 - F::cast_from(0.27785333333333333334e0_f64) * t58356 - F::new(0.375102e1) * t58360 + F::new(0.83356e0) * t58363 - F::cast_from(0.13892666666666666667e0_f64) * t58367 + F::cast_from(0.68863333333333333332e0_f64) * t52389 + F::new(0.41318e1) * t52391 + F::cast_from(0.76514814814814814814e0_f64) * t52393;
    (t59263, t59281)
}
