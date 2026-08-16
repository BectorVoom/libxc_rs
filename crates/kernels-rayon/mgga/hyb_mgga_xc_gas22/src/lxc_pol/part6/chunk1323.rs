//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1323/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1323(t20643: f64, t2195: f64, t4117: f64, t4121: f64, t6601: f64, t20631: f64, t10566: f64, t2200: f64, t3324: f64, t8682: f64, t10577: f64, t2194: f64, t791: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28877 = t20643 * t4117 * t2195;
    let t28880 = t6601 * t4121 * t2195;
    let t28883 = t20631 * t4117 * t2195;
    let t28885 = t10566 * t2200;
    let t28887 = t3324 * t8682;
    let t28890 = t2194 * t10577 * t791;
    (t28877, t28880, t28883, t28885, t28887, t28890)
}
