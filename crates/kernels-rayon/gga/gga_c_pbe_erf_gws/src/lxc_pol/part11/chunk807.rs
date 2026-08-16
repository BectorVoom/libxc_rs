//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 807/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk807(t10634: f64, t12622: f64, t12625: f64, t12629: f64, t12633: f64, t12637: f64, t12641: f64, t12645: f64, t12649: f64, t12653: f64, t12655: f64, t12656: f64, t12658: f64, t12662: f64, t12705: f64, t12707: f64, t12713: f64, t7541: f64) -> f64 {
    let t13016 = -t12622 + t12625 + t12629 + t12633 - t12637 - 2.0_f64 / 15.0_f64 * t10634 + t12641 + t12645 + t12649 + t12653 - t12655 - 2.0_f64 / 9.0_f64 * t7541 - t12656 - t12658 - t12662 - t12705 + t12707 + t12713;
    t13016
}
