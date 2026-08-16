//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 894/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk894(t1306: f64, t135: f64, t273: f64, t2993: f64, t2997: f64, t805: f64, t9132: f64, t9134: f64, t9136: f64, t9207: f64, t9209: f64, t9211: f64, t9213: f64, t9215: f64, t9218: f64, t9221: f64, t9224: f64, t9227: f64, t9231: f64, t9234: f64, t9238: f64, t9244: f64, t9247: f64, t9716: f64) -> f64 {
    let t9720 = t135 * t273 * t805 * t9716 - 2.0_f64 * t1306 * t2993 * t2997 - t9132 - t9134 + t9136 - t9207 + t9209 - t9211 - t9213 + t9215 + t9218 - t9221 - t9224 - t9227 + t9231 + t9234 + t9238 - t9244 - t9247;
    t9720
}
