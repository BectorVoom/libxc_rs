//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 409/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk409<F: Float>(t160: F, t358: F, t363: F, t558: F, t2221: F, t1901: F, t2135: F, t2140: F, t2144: F, t2159: F, t2164: F, t2165: F, t2167: F, t2170: F, t2174: F, t2182: F, t2187: F, t2192: F, t2195: F, t2198: F, t2202: F, t2207: F, t2214: F, t2218: F, t28: F, t446: F, t89: F) -> (F, F, F, F) {
    let t2222 = t160 * t358;
    let t2223 = t363 * t558;
    let t2224 = t2222 * t2223;
    let t2225 = t2221 * t2224;
    let t2228 = t89 * t28 * t2135 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2140 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t2144 - t446 * t2159 / F::cast_from(3.0_f64) + t2164 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2165 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2167 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t2170 - t446 * t2174 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t2182 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t2187 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t2192 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t2195 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t446 * t2198 - t446 * t2202 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t446 * t2207 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t2214 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t446 * t2218 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t2225;
    (t2222, t2224, t2225, t2228)
}
