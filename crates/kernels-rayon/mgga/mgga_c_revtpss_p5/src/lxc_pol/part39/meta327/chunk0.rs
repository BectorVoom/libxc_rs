//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1107/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1107(t1175: f64, t3520: f64, t3519: f64, t444: f64, t439: f64, t3495: f64, t3515: f64, t3523: f64, t1156: f64, t3451: f64, t12295: f64, t12351: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12481 = t1175 * t3520;
    let t12485 = 1.0_f64 / t3519 / t444;
    let t12486 = t439 * t12485;
    let t12491 = t1175 * t3495;
    let t12500 = t3515 * t3523;
    let t12511 = t1156 * t3451;
    let t12542 = 0.93932222222222222223e0_f64 * t12295;
    let t12543 = 0.36793333333333333333e0_f64 * t12351;
    (t12481, t12485, t12486, t12491, t12500, t12511, t12542, t12543)
}
