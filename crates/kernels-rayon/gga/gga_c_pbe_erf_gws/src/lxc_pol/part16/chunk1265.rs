//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1265/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1265(t14031: f64, t9604: f64, t3116: f64, t51237: f64, t14069: f64, t9108: f64, t14570: f64, t6217: f64, t1125: f64, t51335: f64, t14535: f64, t2087: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54002 = t14031 * t9604;
    let t54004 = t3116 * t51237;
    let t54006 = t9108 * t14069;
    let t54008 = t6217 * t14570;
    let t54010 = t1125 * t51335;
    let t54012 = t2087 * t14535;
    (t54002, t54004, t54006, t54008, t54010, t54012)
}
