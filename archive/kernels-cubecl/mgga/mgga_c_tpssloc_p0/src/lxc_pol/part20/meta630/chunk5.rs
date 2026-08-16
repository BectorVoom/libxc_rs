//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2289/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2289<F: Float>(t47276: F, t13176: F, t2696: F, t849: F, t13222: F, t13228: F, t13251: F, t13300: F, t13306: F, t13350: F, t2643: F, t2645: F, t2647: F, t2679: F, t41063: F, t41090: F, t4178: F, t4248: F, t4250: F, t47012: F, t47262: F, t47267: F, t47270: F, t47271: F, t47273: F, t9627: F, t9642: F, t9653: F, t9958: F) -> F {
    let t47277 = F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t47276;
    let t47278 = t13176 * t2696;
    let t47279 = t47278 * t849;
    let t47281 = t2643 * t2645 * t13300 * t2679 / F::cast_from(256.0_f64) + t9642 * t13306 / F::cast_from(256.0_f64) + t2643 * t2645 * t4248 * t9958 / F::cast_from(768.0_f64) + t41063 * t4250 / F::cast_from(256.0_f64) + t13251 * t9653 / F::cast_from(256.0_f64) - t4178 * t13222 * t13228 * t41090 / F::cast_from(128.0_f64) + F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t4178 * t13350 * t47012 * t9627 + t2643 * t13222 * t47262 * t2647 / F::cast_from(256.0_f64) - F::cast_from(35.0_f64) / F::cast_from(384.0_f64) * t47267 - t47270 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t47271 + F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t47273 - t47277 + F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t47279;
    t47281
}
