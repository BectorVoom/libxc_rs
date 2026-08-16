//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 603/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk603(t488: f64, t8355: f64, t83: f64, t1841: f64, t487: f64, t492: f64, t1820: f64, t1825: f64, t1851: f64, t1853: f64, t379: f64, t1909: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8356 = t488 * t8355;
    let t8357 = t83 * t8356;
    let t8360 = t1841 * t487;
    let t8361 = t8360 * t492;
    let t8362 = t83 * t8361;
    let t8364 = t1825 * t1820;
    let t8365 = t83 * t8364;
    let t8367 = t1851 * t1853;
    let t8368 = t8367 * t379;
    let t8369 = t1909 * t8368;
    (t8356, t8357, t8360, t8361, t8362, t8364, t8365, t8367, t8368, t8369)
}
