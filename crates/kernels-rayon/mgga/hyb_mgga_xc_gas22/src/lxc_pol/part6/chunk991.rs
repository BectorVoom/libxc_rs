//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 991/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk991(t1006: f64, t9195: f64, t3557: f64, t997: f64, t1007: f64, t2594: f64, t3560: f64, t8988: f64, t8992: f64, t8995: f64, t8999: f64, t9101: f64, t9103: f64, t9106: f64, t9108: f64, t9110: f64, t9170: f64, t998: f64) -> (f64, f64, f64) {
    let t9196 = t9195 * t1006;
    let t9199 = t3557 * t997;
    let t9204 = t8988 - t8992 - t8995 - t8999 - t9101 - t9103 - t9106 - t9108 - t9110 - t9170 + 0.5848223622634646207e0_f64 * t998 * t9196 + 0.11696447245269292414e1_f64 * t9199 * t1007 + 0.5848223622634646207e0_f64 * t3560 * t2594;
    (t9196, t9199, t9204)
}
