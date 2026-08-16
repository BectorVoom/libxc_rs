//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1237/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1237(t121: f64, t124: f64, t1268: f64, t13124: f64, t13129: f64, t13136: f64, t16287: f64, t16361: f64, t16364: f64, t16367: f64, t2060: f64, t22751: f64, t29284: f64, t3406: f64, t3411: f64, t3412: f64, t38298: f64, t4595: f64, t4643: f64, t4646: f64, t48428: f64, t55893: f64, t55933: f64, t56222: f64, t56255: f64, t56256: f64, t56258: f64, t56259: f64, t56262: f64, t56292: f64, t56298: f64, t56310: f64, t641: f64, t9747: f64) -> f64 {
    let t56344 = -0.12897460341341234505e3_f64 * (t56255 + t56256 + t56258 + t56259 + t56262 + t56292 + t56298 + t56310) * t121 * t124 + 0.15476952409609481406e4_f64 * t48428 * t1268 - 0.92861714457656888434e4_f64 * t38298 * t4643 + 0.23215428614414222108e4_f64 * t13124 * t4646 + 0.30953904819218962812e5_f64 * t29284 * t16361 - 0.18572342891531377687e5_f64 * t13129 * t16364 + 0.15476952409609481406e4_f64 * t3406 * t16367 - 0.46430857228828444218e5_f64 * t22751 * t124 * t56222 + 0.46430857228828444218e5_f64 * t9747 * t13136 * t4595 - 0.46430857228828444218e4_f64 * t2060 * t124 * t55893 - 0.61907809638437925624e4_f64 * t3411 * t3412 * t16287 + 0.38692381024023703515e3_f64 * t641 * t124 * t55933;
    t56344
}
