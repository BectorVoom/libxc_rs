//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1877/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1877(t25: f64, t870: f64, t4255: f64, t16596: f64, t22960: f64, t1484: f64, t606: f64, t4119: f64, t7484: f64, t794: f64, t6562: f64, t1887: f64, t23056: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25014 = t870 * t25;
    let t25015 = t25014 * t4255;
    let t25021 = t22960 * t16596;
    let t25024 = t606 * t1484;
    let t25028 = t25 * t4119;
    let t25035 = t794 * t7484;
    let t25036 = t6562 * t25035;
    let t25038 = t23056 * t1887;
    (t25014, t25015, t25021, t25024, t25028, t25035, t25036, t25038)
}
