//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 815/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk815(t5484: f64, t5493: f64, t5519: f64, t5557: f64, t5513: f64, t5516: f64, t5522: f64, t5525: f64, t5539: f64, t5541: f64, t5548: f64, t5551: f64, t5553: f64, t5560: f64, t5563: f64, t5566: f64, t5570: f64, t5574: f64) -> (f64, f64, f64, f64) {
    let t5846 = t5484 * t5493;
    let t5852 = 0.16068111111111111111e1_f64 * t5519;
    let t5859 = 0.46308888888888888888e0_f64 * t5557;
    let t5865 = 0.264729375e1_f64 * t5513 - 0.52945875e1_f64 * t5516 + 0.3529725e1_f64 * t5541 - t5852 + 0.20659e1_f64 * t5522 - 0.1549425e1_f64 * t5525 + 0.1549425e1_f64 * t5539 - 0.157790625e0_f64 * t5548 + 0.94674375e0_f64 * t5551 + 0.6311625e0_f64 * t5553 - t5859 + 0.104195e1_f64 * t5560 - 0.62517e0_f64 * t5563 - 0.62517e0_f64 * t5566 + 0.937755e0_f64 * t5570 + 0.312585e0_f64 * t5574;
    (t5846, t5852, t5859, t5865)
}
