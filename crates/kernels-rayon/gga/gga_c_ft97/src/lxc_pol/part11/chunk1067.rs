//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1067/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1067(t1775: f64, t9949: f64, t2499: f64, t8282: f64, t2344: f64, t2371: f64, t2: f64, t9931: f64, t9917: f64, t9897: f64, t665: f64, t7514: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42105 = t1775 * t9949;
    let t42107 = t8282 * t2499;
    let t42109 = t2344 * t2371;
    let t42110 = t42109 * t2;
    let t42117 = t1775 * t9931;
    let t42119 = t1775 * t9917;
    let t42121 = t1775 * t9897;
    let t42123 = t665 * t7514;
    (t42105, t42107, t42109, t42110, t42117, t42119, t42121, t42123)
}
