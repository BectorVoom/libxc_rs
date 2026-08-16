//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1069/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1069(t12273: f64, t1264: f64, t247: f64, t1284: f64, t3555: f64, t3624: f64, t12803: f64, t3629: f64, t3626: f64, t1121: f64, t3603: f64, t606: f64) -> (f64, f64, f64, f64) {
    let t12828 = t247 * t1264 * t12273;
    let t12831 = t3555 * t1284;
    let t12832 = t12831 * t3624;
    let t12835 = t12803 * t3629;
    let t12836 = t3626 * t12835;
    let t12839 = t3603 * t1121;
    let t12840 = t12839 * t606;
    (t12828, t12832, t12836, t12840)
}
