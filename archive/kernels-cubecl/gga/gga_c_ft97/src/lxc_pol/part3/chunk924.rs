//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 924/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk924<F: Float>(t2: F, t5053: F, t2372: F, t713: F, t4934: F, t9707: F, t3821: F, t3930: F, t13306: F, t13308: F, t13329: F, t13335: F, t13338: F, t13339: F, t13345: F, t13388: F, t13680: F, t13682: F, t13688: F, t18271: F, t18276: F, t18279: F, t18283: F, t18286: F, t462: F, t9907: F, t9935: F, t9936: F) -> F {
    let t18288 = t2 * t5053;
    let t18290 = t2372 * t18288 * t713;
    let t18293 = t2 * t4934;
    let t18295 = t9707 * t18293 * t713;
    let t18299 = t2372 * t3930 * t3821;
    let t18302 = -t13306 + t13308 - t13329 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t13335 - t13338 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t13339 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t9936 + t13345 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t13682 * t18271 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t13688 * t18276 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t13688 * t18279 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t9907 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t18283 - t9935 - t13388 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t13680 + t18286 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) * t462 * t18290 - F::cast_from(6.0_f64) * t462 * t18295 + F::cast_from(4.0_f64) * t462 * t18299;
    t18302
}
