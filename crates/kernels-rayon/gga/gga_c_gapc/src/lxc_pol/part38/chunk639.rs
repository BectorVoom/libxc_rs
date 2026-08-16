//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 639/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk639(t1096: f64, t2469: f64, t3265: f64, t338: f64, t3656: f64, t3658: f64, t3661: f64, t3722: f64, t3742: f64, t3746: f64, t3795: f64, t884: f64) -> f64 {
    let t3797 = -2.0_f64 * t1096 * t3265 + 2.0_f64 * t2469 * t3746 + t338 * t3742 - t3795 * t884 - t3656 + t3658 - t3661 + t3722;
    t3797
}
