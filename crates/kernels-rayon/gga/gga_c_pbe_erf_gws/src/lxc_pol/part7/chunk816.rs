//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 816/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk816(t6618: f64, t6623: f64, t6625: f64, t6650: f64, t6654: f64, t6663: f64, t6676: f64, t6682: f64, t6692: f64, t6696: f64, t6700: f64, t6704: f64, t6709: f64, t6713: f64) -> f64 {
    let t6736 = t6618 + t6623 - t6625 + t6650 + t6654 - t6663 + t6676 + t6682 + t6692 - t6696 - t6700 - t6704 - t6709 - t6713;
    t6736
}
