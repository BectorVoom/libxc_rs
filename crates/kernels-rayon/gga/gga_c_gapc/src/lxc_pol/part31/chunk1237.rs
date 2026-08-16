//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1237/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1237(t11384: f64, t26759: f64, t26836: f64, t11499: f64, t1700: f64, t633: f64, t1040: f64, t3687: f64, t8863: f64, t3115: f64, t436: f64, t8780: f64) -> (f64, f64, f64, f64, f64) {
    let t34565 = t11384 * t26759;
    let t34567 = t11384 * t26836;
    let t34570 = t633 * t11499 * t1700;
    let t34573 = t8863 * t3687 * t1040;
    let t34576 = t3115 * t436 * t8780;
    (t34565, t34567, t34570, t34573, t34576)
}
