//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1084/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1084(t11810: f64, t11812: f64, t11816: f64, t11818: f64, t11833: f64, t11838: f64, t11844: f64, t11862: f64, t11863: f64, t11867: f64, t11870: f64, t11874: f64) -> f64 {
    let t12156 = -t11810 + t11812 - t11816 - t11818 + t11833 + t11838 + t11844 + t11862 - t11863 - t11867 + t11870 - t11874;
    t12156
}
