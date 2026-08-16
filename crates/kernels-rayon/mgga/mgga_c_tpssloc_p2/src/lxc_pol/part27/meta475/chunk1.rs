//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1845/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1845(t23476: f64, t343: f64, t23562: f64, t1046: f64, t1935: f64, t23533: f64, t23537: f64, t23541: f64, t23544: f64, t23548: f64, t23551: f64, t23554: f64, t23557: f64, t23560: f64, t3043: f64, t3134: f64, t3153: f64, t378: f64, t6717: f64, t6747: f64) -> (f64, f64, f64) {
    let t23563 = t23476 * t343;
    let t23564 = t23562 * t23563;
    let t23569 = t23533 / 1728.0_f64 + t23537 * t3134 / 768.0_f64 - t23541 * t3043 / 1536.0_f64 + t23544 * t1046 / 1152.0_f64 - 0.10093189023535097714e-3_f64 * t1935 * t23548 - t23551 * t378 / 144.0_f64 + t23554 / 1152.0_f64 + 19.0_f64 / 864.0_f64 * t23557 * t378 - t23560 / 216.0_f64 - 0.20186378047070195428e-3_f64 * t23564 * t6747 - t6717 * t3153 / 144.0_f64;
    (t23563, t23564, t23569)
}
