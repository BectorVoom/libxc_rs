//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1385/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1385(t33865: f64, t33870: f64, t33872: f64, t33875: f64, t33878: f64, t33881: f64, t33885: f64, t33888: f64, t33897: f64, t33899: f64, t33902: f64, t33904: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36738 = 0.3077456993052877797e-8_f64 * t33865;
    let t36740 = 0.19336232562226912508e-7_f64 * t33870;
    let t36741 = 0.42205124476153752644e-7_f64 * t33872;
    let t36742 = 0.78582449132890172432e-8_f64 * t33875;
    let t36743 = 0.20240885416666666668e-4_f64 * t33878;
    let t36744 = 0.57920616843011475696e-5_f64 * t33881;
    let t36745 = 0.8446756622939173539e-6_f64 * t33885;
    let t36746 = 0.13493923611111111112e-4_f64 * t33888;
    let t36749 = 0.58364997692245511715e-8_f64 * t33897;
    let t36750 = 0.21102562238076876322e-7_f64 * t33899;
    let t36751 = 0.2748593934505475288e-6_f64 * t33902;
    let t36752 = 0.36652500116630512966e-6_f64 * t33904;
    (t36738, t36740, t36741, t36742, t36743, t36744, t36745, t36746, t36749, t36750, t36751, t36752)
}
