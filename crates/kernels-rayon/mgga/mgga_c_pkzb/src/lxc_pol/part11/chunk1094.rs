//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1094/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1094(t2609: f64, t5146: f64, t1542: f64, t2605: f64, t16613: f64, t16619: f64, t16621: f64, t1009: f64, t4803: f64, t5142: f64, t1639: f64, t7035: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19710 = t2609 * t5146;
    let t19742 = t1542 * t2605;
    let t19743 = 60.0_f64 * t19742;
    let t19748 = 240.0_f64 * t16613;
    let t19751 = 36.0_f64 * t16619;
    let t19752 = 96.0_f64 * t16621;
    let t19754 = t4803 * t1009;
    let t19756 = t5142 * t1009;
    let t19757 = 144.0_f64 * t19756;
    let t19758 = t7035 * t1639;
    (t19710, t19743, t19748, t19751, t19752, t19754, t19757, t19758)
}
