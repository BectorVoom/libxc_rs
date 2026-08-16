//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 874/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk874(t3591: f64, t5493: f64, t721: f64, t2801: f64, t2820: f64, t5845: f64, t7324: f64, t7486: f64, t9213: f64, t9215: f64, t9218: f64, t9221: f64, t9224: f64, t9227: f64, t9231: f64, t9234: f64, t9238: f64) -> (f64, f64, f64) {
    let t9401 = t3591 * t5493;
    let t9402 = t9401 * t721;
    let t9409 = 0.10254018858216406658e4_f64 * t5845 * t9402 + t9213 - t9215 - t9218 + t9221 + t9224 + t9227 - t9231 - t9234 - t9238 - 4.0_f64 * t7486 * t2801 + 0.64327917994770140268e2_f64 * t7324 * t2820;
    (t9401, t9402, t9409)
}
