//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2428/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2428(t3566: f64, t5462: f64, t5477: f64, t576: f64, t588: f64, t15: f64, t27: f64, t11: f64, t22: f64, t10276: f64, t584: f64, t596: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45859 = t3566 * t5462;
    let t45863 = t3566 * t5477;
    let t45928 = t576 * t588;
    let t45931 = 120.0_f64 * t15 * t27;
    let t45933 = 24.0_f64 * t11 * t22;
    let t45934 = t10276 * t588;
    let t45938 = t584 * t596;
    (t45859, t45863, t45928, t45931, t45933, t45934, t45938)
}
