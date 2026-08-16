//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2011/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2011(t268: f64, t521: f64, t9799: f64, t9847: f64, t677: f64, t9494: f64, t3684: f64, t12110: f64, t9885: f64, t2505: f64, t2527: f64, t1294: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39321 = t521 * t268;
    let t39322 = t9799 * t9847;
    let t39324 = 0.1301229756036208781e0_f64 * t39321 * t39322;
    let t39325 = t677 * t9494;
    let t39327 = 0.38025319932552508021e2_f64 * t3684 * t39325;
    let t39328 = t12110 * t9885;
    let t39336 = t2527 * t2505;
    let t39338 = 0.21053605041484726346e2_f64 * t1294 * t39336;
    (t39321, t39322, t39324, t39325, t39327, t39328, t39336, t39338)
}
