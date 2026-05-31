//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 832/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk832<F: Float>(t12001: F, t3471: F, t1060: F, t1647: F, t569: F, t13040: F, t13042: F, t13045: F, t13049: F, t13051: F, t13055: F, t13058: F, t13062: F, t13067: F, t13072: F, t1901: F, t446: F, t9321: F, t9340: F, t9342: F) -> F {
    let t13075 = t12001 * t3471;
    let t13078 = t569 * t1060 * t1647;
    let t13081 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9321 - t13040 - t13042 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t13045 - t13049 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t13051 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t13055 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t13058 + t13062 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t9340 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t9342 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t13067 - F::cast_from(2.0_f64) * t446 * t13072 + F::cast_from(22.0_f64) / F::cast_from(27.0_f64) * t13075 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t446 * t13078;
    t13081
}
