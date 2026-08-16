//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 903/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk903(t2530: f64, t4758: f64, t2529: f64, t6052: f64, t280: f64, t6056: f64, t1444: f64, t309: f64, t310: f64, t5470: f64, t4754: f64, t1625: f64) -> (f64, f64, f64, f64, f64) {
    let t9585 = t2530 * t4758;
    let t9588 = t6052 * t2529;
    let t9589 = t6056 * t280;
    let t9590 = t9588 * t9589;
    let t9592 = t309 * t310 * t1444;
    let t9595 = t5470 * t2529;
    let t9596 = t9595 * t4758;
    let t9599 = t2530 * t4754;
    let t9600 = t9599 * t1625;
    (t9585, t9590, t9592, t9596, t9600)
}
