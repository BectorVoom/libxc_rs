//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1114/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1114(t11872: f64, t9990: f64, t11356: f64, t28472: f64, t9574: f64, t33770: f64, t33772: f64, t33774: f64, t33777: f64, t33779: f64, t33784: f64, t33787: f64, t33789: f64, t33791: f64) -> f64 {
    let t33793 = t11872 * t9990;
    let t33796 = t9574 * t11356 * t28472;
    let t33798 = -0.52838066223730378166e-7_f64 * t33770 - 0.20010856351627032588e-7_f64 * t33772 - 0.20047434126173032506e-6_f64 * t33774 + 0.33147827249531850014e-7_f64 * t33777 - 0.28985453471303521737e-5_f64 * t33779 - 0.96681162811134562541e-9_f64 * t33784 + 0.1422820120100248667e-7_f64 * t33787 + 0.17391272082782113042e-4_f64 * t33789 - 0.21102562238076876322e-7_f64 * t33791 + 0.16882049790461501058e-6_f64 * t33793 + 0.10551281119038438161e-7_f64 * t33796;
    t33798
}
