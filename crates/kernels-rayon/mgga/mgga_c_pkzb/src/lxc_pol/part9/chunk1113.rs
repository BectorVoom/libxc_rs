//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1113/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1113(t18509: f64, t369: f64, t6287: f64, t858: f64, t2277: f64, t356: f64, t2280: f64, t2099: f64, t3235: f64, t6386: f64, t2387: f64, t824: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18878 = t369 * t18509;
    let t18882 = t858 * t6287;
    let t18885 = t2277 * t2277;
    let t18887 = t356 / t18885;
    let t18888 = t2280 * t2280;
    let t18889 = 1.0_f64 / t18888;
    let t18940 = t3235 * t2099 * t6386;
    let t18957 = t824 * t2387;
    (t18878, t18882, t18887, t18889, t18940, t18957)
}
