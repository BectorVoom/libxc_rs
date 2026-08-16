//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1293/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1293(t14069: f64, t9108: f64, t14570: f64, t6217: f64, t1125: f64, t51335: f64, t14535: f64, t2087: f64, t3291: f64, t51214: f64, t14007: f64, t9485: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54006 = t9108 * t14069;
    let t54008 = t6217 * t14570;
    let t54010 = t1125 * t51335;
    let t54012 = t2087 * t14535;
    let t54014 = t51214 * t3291;
    let t54015 = 7.0_f64 / 576.0_f64 * t54014;
    let t54016 = t14007 * t9485;
    (t54006, t54008, t54010, t54012, t54015, t54016)
}
