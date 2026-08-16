//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 557/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk557(t2172: f64, t2221: f64, t2175: f64, t2187: f64, t2205: f64, t2210: f64, t2216: f64, t2218: f64, t2224: f64, t2228: f64, t2232: f64) -> (f64, f64, f64) {
    let t2303 = 0.40256666666666666667e0_f64 * t2172;
    let t2308 = 0.137975e0_f64 * t2221;
    let t2312 = -0.1294625e1_f64 * t2205 + 0.258925e1_f64 * t2210 + t2303 - 0.60385e0_f64 * t2175 + 0.905775e0_f64 * t2187 + 0.82524375e-1_f64 * t2216 + 0.16504875e0_f64 * t2218 + t2308 - 0.33114e0_f64 * t2224 + 0.248355e0_f64 * t2228 + 0.248355e0_f64 * t2232;
    (t2303, t2308, t2312)
}
