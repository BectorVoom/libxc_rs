//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1044/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1044(t1154: f64, t6455: f64, t254: f64, t9404: f64, t906: f64, t3261: f64, t6416: f64, t2074: f64, t274: f64, t1123: f64, t2255: f64, t2338: f64, t3252: f64) -> (f64, f64, f64, f64, f64) {
    let t9457 = t6455 * t1154;
    let t9459 = t254 * t9404;
    let t9460 = t9459 * t906;
    let t9464 = 7.0_f64 / 576.0_f64 * t6416 * t3261;
    let t9465 = t274 * t2074;
    let t9467 = t2255 * t1123 * t9465;
    let t9470 = t3252 * t2338;
    (t9457, t9460, t9464, t9467, t9470)
}
