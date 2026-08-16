//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1141/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1141(t14565: f64, t14567: f64, t1135: f64, t3065: f64, t2134: f64, t14059: f64, t14073: f64, t14080: f64, t14085: f64, t14554: f64, t14556: f64, t14558: f64, t14560: f64, t14563: f64) -> (f64, f64) {
    let t14568 = t14565 * t14567;
    let t14570 = t3065 * t1135;
    let t14571 = t2134 * t14570;
    let t14574 = 7.0_f64 / 288.0_f64 * t14554 - t14556 / 384.0_f64 + 7.0_f64 / 576.0_f64 * t14558 - t14560 / 192.0_f64 + 7.0_f64 / 576.0_f64 * t14059 + 7.0_f64 / 144.0_f64 * t14563 + t14568 / 96.0_f64 - t14571 / 96.0_f64 + t14073 + 7.0_f64 / 1152.0_f64 * t14080 + t14085;
    (t14570, t14574)
}
