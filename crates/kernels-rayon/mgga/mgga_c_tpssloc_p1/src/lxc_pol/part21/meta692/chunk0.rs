//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2507/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2507(t13176: f64, t2696: f64, t849: f64, t13360: f64, t2707: f64, t1509: f64, t9975: f64, t242: f64, t41347: f64, t812: f64, t13297: f64, t9573: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47278 = t13176 * t2696;
    let t47279 = t47278 * t849;
    let t47283 = t13360 * t2707;
    let t47285 = t1509 * t9975;
    let t47307 = t812 * t41347 * t242;
    let t47333 = t9573 * t13297;
    (t47278, t47279, t47283, t47285, t47307, t47333)
}
