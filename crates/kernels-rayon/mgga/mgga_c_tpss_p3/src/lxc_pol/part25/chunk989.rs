//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 989/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk989(t5458: f64, t9895: f64, t12758: f64, t177: f64, t5343: f64, t737: f64, t3205: f64, t10016: f64, t5328: f64, t9924: f64, t3217: f64, t4578: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13627 = t5458 * t9895;
    let t13631 = 0.23392894490538584828e1_f64 * t12758;
    let t13635 = t5343 * t177;
    let t13636 = t13635 * t737;
    let t13637 = 0.5848223622634646207e0_f64 * t13636;
    let t13641 = t5458 * t3205;
    let t13645 = 12.0_f64 * t10016;
    let t13646 = t9924 * t5328;
    let t13651 = t3217 * t4578;
    (t13627, t13631, t13637, t13641, t13645, t13646, t13651)
}
