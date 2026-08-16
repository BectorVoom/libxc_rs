//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1347/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1347(t120576: f64, t114253: f64, t114255: f64, t2007: f64, t254: f64, t114278: f64, t32694: f64, t6914: f64, t114291: f64, t32735: f64, t6883: f64, t114296: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t120577 = 0.82246703342411321825e-2_f64 * t120576;
    let t120579 = 0.38381794893125283518e-1_f64 * t114253;
    let t120590 = 0.76763589786250567036e-1_f64 * t114255;
    let t120591 = t2007 * t254;
    let t120594 = 0.16449340668482264365e-1_f64 * t114278;
    let t120605 = t6914 * t32694;
    let t120606 = 0.76763589786250567037e-1_f64 * t120605;
    let t120607 = 0.38381794893125283518e-1_f64 * t114291;
    let t120610 = t6883 * t32735;
    let t120611 = 0.38381794893125283518e-1_f64 * t120610;
    let t120612 = 0.38381794893125283518e-1_f64 * t114296;
    (t120577, t120579, t120590, t120591, t120594, t120606, t120607, t120611, t120612)
}
