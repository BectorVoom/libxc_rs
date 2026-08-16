//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1053/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1053(t20487: f64, t3141: f64, t3131: f64, t3137: f64, t15341: f64, t8676: f64, t128: f64, t1453: f64, t134: f64, t681: f64, t5216: f64, t1673: f64, t9255: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26578 = t3141 * t20487;
    let t26597 = t3131 * t3137;
    let t26609 = t8676 * t15341;
    let t26662 = t128 * t1453;
    let t26697 = t681 * t134;
    let t26698 = t26697 * t5216;
    let t26759 = t1673 * t9255;
    (t26578, t26597, t26609, t26662, t26698, t26759)
}
