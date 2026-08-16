//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1290/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1290(t13859: f64, t14682: f64, t56296: f64, t6287: f64, t15161: f64, t2397: f64, t12074: f64, t3079: f64, t14135: f64, t3912: f64, t51913: f64, t11505: f64, t3972: f64, t3975: f64) -> (f64, f64, f64, f64, f64) {
    let t56586 = t13859 * t14682 * t56296 * t6287;
    let t56588 = t15161 * t2397;
    let t56590 = t12074 * t3079;
    let t56593 = t3912 * t14135 * t51913;
    let t56596 = t3972 * t3975 * t11505;
    (t56586, t56588, t56590, t56593, t56596)
}
