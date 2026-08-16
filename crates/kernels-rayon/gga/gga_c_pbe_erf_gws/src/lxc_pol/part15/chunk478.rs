//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 478/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk478(t1739: f64, t1742: f64, t1752: f64, t1777: f64, t1780: f64, t1785: f64, t1789: f64, t1797: f64, t1800: f64, t1808: f64, t1814: f64, t1819: f64) -> f64 {
    let t2012 = -t1739 - t1742 + t1752 + t1777 - t1780 + t1785 + t1789 + t1797 + t1800 + t1808 + t1814 - t1819;
    t2012
}
