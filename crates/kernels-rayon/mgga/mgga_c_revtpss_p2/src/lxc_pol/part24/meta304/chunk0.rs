//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1089/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1089(t21451: f64, t460: f64, t1811: f64, t3781: f64, t1770: f64, t5462: f64, t473: f64, t6695: f64, t5477: f64, t20849: f64, t487: f64, t5812: f64, t602: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21452 = t460 * t21451;
    let t21455 = t3781 * t1811;
    let t21456 = t460 * t21455;
    let t21500 = t1770 * t5462;
    let t21541 = t473 * t6695;
    let t21579 = t1770 * t5477;
    let t21621 = t20849 * t487;
    let t21663 = t5812 * t602;
    (t21452, t21455, t21456, t21500, t21541, t21579, t21621, t21663)
}
