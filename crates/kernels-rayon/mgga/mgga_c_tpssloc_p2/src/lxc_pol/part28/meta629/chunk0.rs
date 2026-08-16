//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1970/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1970(t1877: f64, t2057: f64, t584: f64, t9212: f64, t2219: f64, t7110: f64, t26756: f64, t86732: f64, t86843: f64, t86868: f64, t86870: f64, t225: f64, t26722: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t92356 = t1877 * t2057 * t584;
    let t92359 = 3.0_f64 * t1877 * t2057 * t9212;
    let t92362 = 2.0_f64 * t1877 * t7110 * t2219;
    let t92364 = 2.0_f64 * t26756 * t86732;
    let t92375 = 0.76763589786250567036e-1_f64 * t86843;
    let t92382 = 0.15352717957250113407e0_f64 * t86868;
    let t92383 = 0.10417915756705434098e0_f64 * t86870;
    let t92386 = t26722 * t225;
    (t92356, t92359, t92362, t92364, t92375, t92382, t92383, t92386)
}
