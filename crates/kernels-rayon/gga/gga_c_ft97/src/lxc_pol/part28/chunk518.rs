//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 518/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk518(t383: f64, t7857: f64, t1598: f64, t66: f64, t1630: f64, t929: f64, t25: f64, t78: f64, t1593: f64, t2248: f64, t422: f64, t110: f64, t1786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11119 = t7857 * t383;
    let t11120 = t1598 * t66;
    let t11121 = t11119 * t11120;
    let t11233 = t1630 * t929;
    let t11240 = t78 * t25;
    let t11247 = t1593 * t929;
    let t11280 = t2248 * t422;
    let t11468 = t1786 * t110;
    (t11119, t11120, t11121, t11233, t11240, t11247, t11280, t11468)
}
