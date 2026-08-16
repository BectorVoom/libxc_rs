//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 232/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk232(t172: f64, t391: f64, t68: f64, t72: f64, t67: f64, t9: f64, t47: f64, t625: f64, t173: f64, t424: f64, t419: f64, t23: f64, t358: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1720 = t391 * t172;
    let t1722 = t68 * t1720 * t72;
    let t1725 = t9 * t67 * t391;
    let t1728 = t47 * t625;
    let t1730 = t68 * t1728 * t72;
    let t1731 = 0.42562405586419753087e-2_f64 * t1730;
    let t1732 = t173 * t424;
    let t1733 = t419 * t1732;
    let t1736 = 1.0_f64 / t23 / t358;
    (t1722, t1725, t1730, t1731, t1733, t1736)
}
