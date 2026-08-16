//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 947/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk947(t10313: f64, t220: f64, t186: f64, t616: f64, t1019: f64, t2826: f64, t7048: f64, t995: f64, t2561: f64, t5218: f64, t10392: f64, t562: f64, t7055: f64) -> (f64, f64, f64, f64) {
    let t10664 = -t10313;
    let t10665 = t220 * t10664;
    let t10666 = t186 * t10665;
    let t10668 = 4.0_f64 / 15.0_f64 * t616 * t10666;
    let t10670 = 4.0_f64 / 15.0_f64 * t2826 * t1019;
    let t10671 = t7048 * t995;
    let t10672 = t10671 * t2561;
    let t10674 = 16.0_f64 / 27.0_f64 * t5218 * t10672;
    let t10676 = t7055 * t10392 * t562;
    (t10668, t10670, t10674, t10676)
}
