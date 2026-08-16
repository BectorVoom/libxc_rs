//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1237/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1237<F: Float>(t121: F, t124: F, t1268: F, t13124: F, t13129: F, t13136: F, t16287: F, t16361: F, t16364: F, t16367: F, t2060: F, t22751: F, t29284: F, t3406: F, t3411: F, t3412: F, t38298: F, t4595: F, t4643: F, t4646: F, t48428: F, t55893: F, t55933: F, t56222: F, t56255: F, t56256: F, t56258: F, t56259: F, t56262: F, t56292: F, t56298: F, t56310: F, t641: F, t9747: F) -> F {
    let t56344 = -F::cast_from(0.12897460341341234505e3_f64) * (t56255 + t56256 + t56258 + t56259 + t56262 + t56292 + t56298 + t56310) * t121 * t124 + F::cast_from(0.15476952409609481406e4_f64) * t48428 * t1268 - F::cast_from(0.92861714457656888434e4_f64) * t38298 * t4643 + F::cast_from(0.23215428614414222108e4_f64) * t13124 * t4646 + F::cast_from(0.30953904819218962812e5_f64) * t29284 * t16361 - F::cast_from(0.18572342891531377687e5_f64) * t13129 * t16364 + F::cast_from(0.15476952409609481406e4_f64) * t3406 * t16367 - F::cast_from(0.46430857228828444218e5_f64) * t22751 * t124 * t56222 + F::cast_from(0.46430857228828444218e5_f64) * t9747 * t13136 * t4595 - F::cast_from(0.46430857228828444218e4_f64) * t2060 * t124 * t55893 - F::cast_from(0.61907809638437925624e4_f64) * t3411 * t3412 * t16287 + F::cast_from(0.38692381024023703515e3_f64) * t641 * t124 * t55933;
    t56344
}
