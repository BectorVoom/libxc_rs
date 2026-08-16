//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 784/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk784(t21428: f64, t21462: f64, t661: f64, t1168: f64, t4934: f64, t2574: f64, t762: f64, t1131: f64, t5053: f64, t265: f64, t3977: f64, t5073: f64, t729: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21463 = t21428 + t21462;
    let t21464 = t661 * t21463;
    let t21472 = t4934 * t1168;
    let t21474 = t2574 * t762 * t21472;
    let t21477 = t1131 * t5053;
    let t21479 = t2574 * t265 * t21477;
    let t21483 = t729 * t3977 * t5073;
    (t21463, t21464, t21472, t21474, t21477, t21479, t21483)
}
