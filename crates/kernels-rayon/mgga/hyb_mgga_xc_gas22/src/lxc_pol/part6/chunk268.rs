//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 268/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk268(t7: f64, t220: f64, t291: f64, t771: f64, t861: f64, t295: f64, t313: f64, t321: f64, t303: f64, t120: f64, t306: f64, t122: f64, t309: f64, dens_threshold: f64, rho0: f64, tau0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t865 = piecewise3(t9, 0.0_f64, t220 * t861 / 2.0_f64 + t771 * t291 / 2.0_f64);
    let t870 = t295 * t313;
    let t871 = 1.0_f64 / t321;
    let t875 = t303 * tau0;
    let t880 = t306 * t120;
    let t883 = t309 * t122;
    (t865, t870, t871, t875, t880, t883)
}
