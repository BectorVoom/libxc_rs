//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1137/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1137<F: Float>(t23581: F, t23583: F, t23585: F, t23587: F, t23592: F, t23597: F, t23602: F, t23605: F, t23608: F, t23612: F, t23614: F, t23616: F) -> F {
    let t23745 = -F::cast_from(0.78666666666666666667e2_f64) * t23581 - F::cast_from(0.52444444444444444446e3_f64) * t23583 + F::cast_from(0.20977777777777777778e3_f64) * t23585 - F::cast_from(0.12586666666666666667e4_f64) * t23587 + F::cast_from(0.94399999999999999998e3_f64) * t23592 - F::cast_from(0.10488888888888888889e3_f64) * t23597 - F::cast_from(0.20977777777777777778e3_f64) * t23602 - F::cast_from(17446.0_f64) * t23605 - F::cast_from(0.14538333333333333333e4_f64) * t23608 + F::cast_from(17446.0_f64) * t23612 + F::cast_from(0.38768888888888888889e4_f64) * t23614 + F::cast_from(0.58153333333333333332e4_f64) * t23616;
    t23745
}
