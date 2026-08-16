//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1215/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1215(t11514: f64, t5633: f64, t137: f64, t1743: f64, t190: f64, t33235: f64, t442: f64, t5971: f64, t11484: f64, t1835: f64, t1691: f64, t1040: f64, t34382: f64) -> (f64, f64, f64, f64, f64) {
    let t34995 = t11514 * t5633;
    let t35001 = t1743 * t33235 * t5971 * t190 * t137 * t442;
    let t35003 = t11484 * t1835;
    let t35005 = t11484 * t1691;
    let t35007 = t34382 * t1040;
    (t34995, t35001, t35003, t35005, t35007)
}
