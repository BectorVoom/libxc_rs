//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 752/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk752(t10022: f64, t10089: f64, t10162: f64, t9843: f64, t258: f64, t9974: f64, t10003: f64, t10054: f64, t10122: f64, t10150: f64, t10154: f64, t2331: f64, t2465: f64, t247: f64, t2617: f64, t263: f64, t719: f64, t771: f64, t9512: f64, t9514: f64, t9781: f64, t9839: f64) -> (f64, f64, f64) {
    let t10164 = t9843 + t10022 + t10089 + t10162;
    let t10166 = t9974 * t258;
    let t10174 = -t10164 * t247 - 3.0_f64 * t2331 * t771 - 3.0_f64 * t2465 * t771 - 3.0_f64 * t2617 * t719 - t263 * t9512 - 2.0_f64 * t263 * t9514 - t263 * t9781 + 12.0_f64 * t10003 - 12.0_f64 * t10054 - 2.0_f64 * t10122 - 6.0_f64 * t10150 - 6.0_f64 * t10154 + 2.0_f64 * t10166 + 12.0_f64 * t9839;
    (t10164, t10166, t10174)
}
