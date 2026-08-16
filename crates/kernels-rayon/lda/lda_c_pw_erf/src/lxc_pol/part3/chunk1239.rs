//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1239/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1239(t14657: f64, t14658: f64, t5651: f64, t1: f64, t1664: f64, t322: f64, t415: f64, t767: f64, t133: f64, t14582: f64, t14585: f64, t14588: f64, t14634: f64, t14641: f64, t14644: f64, t14648: f64, t14652: f64, t14656: f64, t1832: f64, t1870: f64) -> (f64, f64, f64, f64, f64) {
    let t14660 = 52.61445_f64 * t14657 * t14658;
    let t14661 = t5651 * t14658;
    let t14666 = t1664 * t1 * t322;
    let t14667 = t415 * t767 * t14666;
    let t14668 = 8.769075_f64 * t14667;
    let t14673 = 1.7881162962962962_f64 * t14582 - 2.2990066666666666_f64 * t14585 + 1.724255_f64 * t14588 - 1.724255_f64 * t133 * t14634 - t14641 - t14644 - t14648 + t14652 + t14656 - t14660 - 62.07318_f64 * t1870 * t14661 + t14668 - 62.07318_f64 * t1870 * t5651 * t1832 * t1664;
    (t14660, t14661, t14666, t14668, t14673)
}
