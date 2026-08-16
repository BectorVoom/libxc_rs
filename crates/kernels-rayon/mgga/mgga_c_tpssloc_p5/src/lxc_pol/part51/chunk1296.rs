//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1296/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1296(t115352: f64, t6897: f64, t6907: f64, t3886: f64, t7213: f64, t225: f64, t31585: f64, t22724: f64, t31569: f64, t31589: f64, t794: f64, t31668: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115601 = t6897 * t115352 * t6907;
    let t115614 = t3886 * t7213;
    let t115619 = t31585 * t225;
    let t115629 = t22724 * t31569;
    let t115630 = 0.26044789391763585244e-1_f64 * t115629;
    let t115658 = t6897 * t794 * t31589;
    let t115774 = t532 * t31668;
    (t115601, t115614, t115619, t115630, t115658, t115774)
}
