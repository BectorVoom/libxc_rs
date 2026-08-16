//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 959/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk959(t7800: f64, t7801: f64, t7803: f64, t7805: f64, t7806: f64, t7808: f64, t7810: f64, t7812: f64, t7833: f64, t7837: f64, t7841: f64, t7843: f64, t7846: f64, t7848: f64, t7850: f64, t7852: f64, t7856: f64) -> f64 {
    let t8453 = t7800 + t7801 - t7803 - t7805 - t7806 - t7808 - t7810 - t7812 + t7833 + t7837 + t7841 + t7843 + t7846 - t7848 - t7850 + t7852 + t7856;
    t8453
}
