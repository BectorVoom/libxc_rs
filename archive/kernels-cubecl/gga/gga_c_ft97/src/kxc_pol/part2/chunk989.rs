//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 989/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk989<F: Float>(t1882: F, t4276: F, t4280: F, t10443: F, t4146: F, t10533: F, t10539: F, t10545: F, t10670: F, t10678: F, t10693: F, t15309: F, t15314: F, t15318: F, t15322: F, t15325: F, t15329: F, t1901: F, t446: F) -> F {
    let t15334 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t4276;
    let t15336 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t4280;
    let t15338 = t10443 * t4146;
    let t15341 = -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t15309 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t15314 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10533 - F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t15318 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t10539 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t10545 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t15322 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t15325 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t15329 + F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t10670 + t10678 / F::cast_from(27.0_f64) + t15334 + t15336 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10693 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t15338;
    t15341
}
