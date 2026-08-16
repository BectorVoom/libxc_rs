//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1261/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1261(t1113: f64, t2182: f64, t3975: f64, t51555: f64, t824: f64, t13917: f64, t51678: f64, t9337: f64, t3959: f64, t8812: f64, t13796: f64, t14423: f64, t2190: f64, t3989: f64) -> (f64, f64, f64, f64) {
    let t53526 = t51555 * t3975 * t1113 * t824 * t2182;
    let t53529 = t13917 * t51678 * t9337;
    let t53531 = t3959 * t8812;
    let t53537 = t3989 * t13796 * t14423 * t2190;
    (t53526, t53529, t53531, t53537)
}
