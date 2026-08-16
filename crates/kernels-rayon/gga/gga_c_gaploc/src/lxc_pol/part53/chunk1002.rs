//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1002/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1002(t38688: f64, t895: f64, t13814: f64, t4953: f64, t1445: f64, t1562: f64, t38613: f64, t874: f64, t40320: f64, t13826: f64, t1580: f64, t46952: f64, t568: f64, t597: f64, t600: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47995 = t895 * t38688;
    let t47997 = t4953 * t13814;
    let t48001 = t1562 * t1445 * t38613 * t874;
    let t48011 = 0.72851559312449424385e1_f64 * t40320;
    let t48013 = 0.23005755572352449806e1_f64 * t1580 * t13826;
    let t48017 = 0.23005755572352449806e1_f64 * t597 * t568 * t600 * t46952;
    (t47995, t47997, t48001, t48011, t48013, t48017)
}
