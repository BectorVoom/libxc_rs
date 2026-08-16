//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 403/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk403(t363: f64, t558: f64, t2222: f64, t2221: f64, t1901: f64, t2135: f64, t2140: f64, t2144: f64, t2159: f64, t2164: f64, t2165: f64, t2167: f64, t2170: f64, t2174: f64, t2182: f64, t2187: f64, t2192: f64, t2195: f64, t2198: f64, t2202: f64, t2207: f64, t2214: f64, t2218: f64, t28: f64, t446: f64, t89: f64) -> (f64, f64, f64, f64) {
    let t2223 = t363 * t558;
    let t2224 = t2222 * t2223;
    let t2225 = t2221 * t2224;
    let t2228 = t89 * t28 * t2135 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t2140 - 2.0_f64 / 3.0_f64 * t446 * t2144 - t446 * t2159 / 3.0_f64 + t2164 + 2.0_f64 / 9.0_f64 * t2165 + 2.0_f64 / 9.0_f64 * t2167 - 2.0_f64 / 3.0_f64 * t446 * t2170 - t446 * t2174 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t2182 + 2.0_f64 / 3.0_f64 * t446 * t2187 + 2.0_f64 / 3.0_f64 * t446 * t2192 + 2.0_f64 / 27.0_f64 * t2195 - 2.0_f64 / 9.0_f64 * t446 * t2198 - t446 * t2202 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t446 * t2207 + 2.0_f64 / 9.0_f64 * t1901 * t2214 + 2.0_f64 / 9.0_f64 * t446 * t2218 + 2.0_f64 / 9.0_f64 * t1901 * t2225;
    (t2223, t2224, t2225, t2228)
}
