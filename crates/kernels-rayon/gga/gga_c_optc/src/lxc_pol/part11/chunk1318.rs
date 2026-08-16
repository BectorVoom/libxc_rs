//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1318/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1318(t241: f64, t57275: f64, t57338: f64, t57383: f64, t57517: f64, t57233: f64, t57236: f64, t57238: f64, t57240: f64, t57244: f64, t57246: f64, t57248: f64, t57251: f64, t57253: f64, t57257: f64, t57260: f64) -> (f64, f64) {
    let t57520 = t241 * (t57275 + t57338 + t57383 + t57517);
    let t57521 = -t57233 - t57236 - t57238 - t57240 - t57244 - t57246 - t57248 - t57251 + t57253 + t57257 + t57260 + t57520;
    (t57520, t57521)
}
