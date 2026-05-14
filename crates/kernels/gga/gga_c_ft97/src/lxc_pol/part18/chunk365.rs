//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 365/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk365<F: Float>(t2222: F, t2223: F, t2221: F, t1901: F, t2135: F, t2140: F, t2144: F, t2159: F, t2164: F, t2165: F, t2167: F, t2170: F, t2174: F, t2182: F, t2187: F, t2192: F, t2195: F, t2198: F, t2202: F, t2207: F, t2214: F, t2218: F, t28: F, t446: F, t89: F) -> (F, F, F) {
    let t2224 = t2222 * t2223;
    let t2225 = t2221 * t2224;
    let t2228 = t89 * t28 * t2135 / 3.0 - 2.0 / 9.0 * t2140 - 2.0 / 3.0 * t446 * t2144 - t446 * t2159 / 3.0 + t2164 + 2.0 / 9.0 * t2165 + 2.0 / 9.0 * t2167 - 2.0 / 3.0 * t446 * t2170 - t446 * t2174 / 3.0 + 2.0 / 3.0 * t446 * t2182 + 2.0 / 3.0 * t446 * t2187 + 2.0 / 3.0 * t446 * t2192 + 2.0 / 27.0 * t2195 - 2.0 / 9.0 * t446 * t2198 - t446 * t2202 / 9.0 - 2.0 / 27.0 * t446 * t2207 + 2.0 / 9.0 * t1901 * t2214 + 2.0 / 9.0 * t446 * t2218 + 2.0 / 9.0 * t1901 * t2225;
    (t2224, t2225, t2228)
}
