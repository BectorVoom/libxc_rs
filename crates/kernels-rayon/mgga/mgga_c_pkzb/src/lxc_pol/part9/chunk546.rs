//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 546/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk546(t2258: f64, t871: f64, t2172: f64, t2221: f64, t2175: f64, t2187: f64, t2205: f64, t2210: f64, t2216: f64, t2218: f64, t2224: f64, t2228: f64, t2232: f64) -> (f64, f64, f64, f64) {
    let t2259 = t2258 * t871;
    let t2264 = 0.68863333333333333333e0_f64 * t2172;
    let t2269 = 0.17365833333333333333e0_f64 * t2221;
    let t2273 = -0.17648625e1_f64 * t2205 + 0.3529725e1_f64 * t2210 + t2264 - 0.103295e1_f64 * t2175 + 0.1549425e1_f64 * t2187 + 0.31558125e0_f64 * t2216 + 0.6311625e0_f64 * t2218 + t2269 - 0.41678e0_f64 * t2224 + 0.312585e0_f64 * t2228 + 0.312585e0_f64 * t2232;
    (t2259, t2264, t2269, t2273)
}
