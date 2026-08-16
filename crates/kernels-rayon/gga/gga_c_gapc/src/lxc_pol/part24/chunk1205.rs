//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1205/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1205(t1388: f64, t2983: f64, t3663: f64, t1030: f64, t33619: f64, t8716: f64, t1971: f64, t505: f64, t8448: f64, t9272: f64, t11356: f64, t26007: f64, t9304: f64) -> (f64, f64, f64, f64, f64) {
    let t35048 = t1388 * t3663 * t2983;
    let t35050 = t1030 * t33619;
    let t35051 = t35050 * t8716;
    let t35056 = t1030 * t1971 * t8448 * t505 * t9272;
    let t35059 = t9304 * t11356 * t26007;
    (t35048, t35050, t35051, t35056, t35059)
}
