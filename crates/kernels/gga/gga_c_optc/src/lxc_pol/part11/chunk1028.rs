//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1028/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1028<F: Float>(t2224: F, t10195: F, t10345: F, t179: F, t182: F, t183: F, t2211: F, t2213: F, t2217: F, t2218: F, t2219: F, t23315: F, t6576: F, t6578: F, t6581: F, t6586: F, t6587: F, t6588: F, t6589: F, t6592: F, t6597: F, t720: F, t723: F, t724: F, t727: F) -> F {
    let t23321 = t2224 * t2224;
    let t23331 = (F::cast_from(0.20106419753086419753e2_f64) * t10195 + F::cast_from(0.20068888888888888889e-1_f64) * t10345) * t183 - F::cast_from(4.0_f64) * t6576 * t723 * t727 + F::cast_from(12.0_f64) * t2211 * t2217 * t2219 - F::cast_from(6.0_f64) * t6578 * t2224 - F::cast_from(24.0_f64) * t720 * t6587 * t6589 + F::cast_from(24.0_f64) * t6581 * t6592 - F::cast_from(4.0_f64) * t2213 * t6597 + F::cast_from(24.0_f64) * t179 / t6586 / t182 * t23315 - F::cast_from(36.0_f64) * t6588 * t2219 * t2224 + F::cast_from(6.0_f64) * t2218 * t23321 + F::cast_from(8.0_f64) * t2218 * t727 * t6597 - t724 * (F::cast_from(0.75383950617283950617e4_f64) * t10195 + F::cast_from(0.12819753086419753086e4_f64) * t10345);
    t23331
}
