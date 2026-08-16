//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1243/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1243(t14469: f64, t50936: f64, t3972: f64, t3975: f64, t9410: f64, t13793: f64, t53229: f64, t13792: f64, t8790: f64, t13776: f64, t37214: f64, t1113: f64, t2182: f64, t51555: f64, t824: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53510 = t50936 * t14469;
    let t53513 = t3972 * t3975 * t9410;
    let t53515 = t53229 * t13793;
    let t53517 = t13792 * t8790;
    let t53520 = t13776 * t3975 * t37214;
    let t53526 = t51555 * t3975 * t1113 * t824 * t2182;
    (t53510, t53513, t53515, t53517, t53520, t53526)
}
