//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1313/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1313(t13984: f64, t56112: f64, t12041: f64, t3037: f64, t353: f64, t376: f64, t51580: f64, t859: f64, t12020: f64, t13917: f64, t13919: f64, t11360: f64, t3959: f64) -> (f64, f64, f64, f64) {
    let t56793 = t56112 * t13984;
    let t56799 = t12041 * t51580 * t859 * t353 * t376 * t3037;
    let t56811 = t13917 * t13919 * t12020;
    let t56813 = t3959 * t11360;
    (t56793, t56799, t56811, t56813)
}
