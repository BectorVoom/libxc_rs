//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 826/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk826(t1441: f64, t41596: f64, t493: f64, t590: f64, t1339: f64, t1537: f64, t34890: f64, t6583: f64, t9537: f64, t10473: f64, t2482: f64, t9263: f64) -> (f64, f64, f64, f64) {
    let t41600 = 0.20449560508757733161e1_f64 * t1441 * t493 * t41596 * t590;
    let t41604 = 0.97135412416599232513e1_f64 * t1537 * t1339 * t41596 * t590;
    let t41606 = t6583 * t34890 * t9537;
    let t41607 = 0.19171462976960374838e1_f64 * t41606;
    let t41609 = t9263 * t10473 * t2482;
    (t41600, t41604, t41607, t41609)
}
