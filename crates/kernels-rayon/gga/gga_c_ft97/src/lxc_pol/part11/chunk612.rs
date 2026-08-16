//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 612/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk612(t494: f64, t8232: f64, t1882: f64, t1897: f64, t1588: f64, t1871: f64, t499: f64, t1893: f64, t454: f64, t1855: f64, t1580: f64, t492: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8475 = t8232 * t494;
    let t8477 = t1882 * t1897;
    let t8480 = t1871 * t499 * t1588;
    let t8483 = t1882 * t1893;
    let t8485 = t8232 * t454;
    let t8487 = t1882 * t1855;
    let t8489 = t1580 * t492;
    (t8475, t8477, t8480, t8483, t8485, t8487, t8489)
}
