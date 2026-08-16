//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1085/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1085(t11876: f64, t11880: f64, t11885: f64, t11888: f64, t11893: f64, t11907: f64, t11911: f64, t11913: f64, t11923: f64, t11927: f64, t9041: f64, t9086: f64, t9096: f64) -> f64 {
    let t12157 = t11876 - t9041 + t11880 + t11885 - t11888 + t11893 + t9086 - t9096 - t11907 + t11911 + t11913 - t11923 + t11927;
    t12157
}
