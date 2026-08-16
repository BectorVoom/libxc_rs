//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 174/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk174(t119: f64, t573: f64, t17: f64, t10: f64, t12: f64, t129: f64, t26: f64, t21: f64, t13: f64, t18: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t574 = t119 * t573;
    let t575 = 14.71989892086604_f64 * t574;
    let t576 = f64::powf(4.0_f64, 1.0_f64 / 12.0_f64);
    let t577 = f64::powf(t17, 1.0_f64 / 12.0_f64);
    let t578 = t577 * t577;
    let t579 = t578 * t578;
    let t580 = t579 * t577;
    let t583 = t576 / t580 * t10;
    let t584 = t12 * t129;
    let t588 = t576 * t576;
    let t589 = t588 * t588;
    let t590 = t589 * t576;
    let t592 = t26 * t26;
    let t593 = t592 * t26;
    let t595 = t21 * t590 / t593;
    let t596 = t13 * t129;
    let t599 = 1.0_f64 / t18;
    (t574, t575, t576, t577, t580, t583, t584, t590, t593, t595, t596, t599)
}
