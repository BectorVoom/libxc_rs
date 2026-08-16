//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2074/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2074(t2157: f64, t43706: f64, t24977: f64, t576: f64, t1395: f64, t7426: f64, t12521: f64, t7467: f64, t81440: f64, t1453: f64, t81439: f64, t26129: f64, t81442: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t86524 = t2157 * t43706;
    let t86557 = t576 * t24977;
    let t86559 = t1395 * t7426;
    let t86582 = 0.135e2_f64 * t12521 * t7467;
    let t86583 = 22.0_f64 / 9.0_f64 * t81440;
    let t86586 = t81439 * t1453;
    let t86588 = t81442 * t26129;
    (t86524, t86557, t86559, t86582, t86583, t86586, t86588)
}
