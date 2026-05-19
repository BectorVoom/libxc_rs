//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1340/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1340<F: Float>(t2927: F, t2934: F, t1062: F, t8785: F, t26252: F, t26258: F, t26278: F, t26280: F, t26284: F, t26289: F, t26293: F, t26296: F, t26300: F, t26304: F, t26306: F) -> (F, F, F) {
    let t26757 = t2927 * t2934;
    let t26760 = t1062 * t8785;
    let t26777 = F::cast_from(0.13734567901234567901e-1_f64) * t26252 + F::cast_from(0.12361111111111111111e0_f64) * t26258 - F::cast_from(0.61805555555555555555e-1_f64) * t26278 + F::cast_from(0.74166666666666666668e-1_f64) * t26280 - F::cast_from(0.22249999999999999999e0_f64) * t26284 + F::cast_from(0.22249999999999999999e0_f64) * t26289 - F::cast_from(0.18541666666666666666e-1_f64) * t26293 + F::new(0.2225e0) * t26296 - F::new(0.33375e0) * t26300 + F::cast_from(0.55625000000000000001e-1_f64) * t26304 - F::cast_from(0.74166666666666666668e-1_f64) * t26306;
    (t26757, t26760, t26777)
}
