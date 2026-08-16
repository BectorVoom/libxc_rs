//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1272/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1272(t114255: f64, t2007: f64, t254: f64, t114278: f64, t32694: f64, t6914: f64, t114291: f64, t32735: f64, t6883: f64, t114296: f64, t114264: f64, t2016: f64, t26224: f64, t26226: f64, t26348: f64, t26472: f64, t26477: f64, t32766: f64, t3758: f64, t40590: f64, t5325: f64, t6958: f64, t6963: f64, t6993: f64, t8475: f64, t91488: f64, t91491: f64) -> f64 {
    let t120590 = 0.76763589786250567036e-1_f64 * t114255;
    let t120591 = t2007 * t254;
    let t120594 = 0.16449340668482264365e-1_f64 * t114278;
    let t120605 = t6914 * t32694;
    let t120606 = 0.76763589786250567037e-1_f64 * t120605;
    let t120607 = 0.38381794893125283518e-1_f64 * t114291;
    let t120610 = t6883 * t32735;
    let t120611 = 0.38381794893125283518e-1_f64 * t120610;
    let t120612 = 0.38381794893125283518e-1_f64 * t114296;
    let t120613 = 24.0_f64 * t26224 * t40590 * t5325 * t8475 - 12.0_f64 * t120591 * t26226 - 2.0_f64 * t2016 * t91488 - 2.0_f64 * t2016 * t91491 + 4.0_f64 * t26348 * t6958 - 2.0_f64 * t26472 * t6958 + 4.0_f64 * t26477 * t6963 - 2.0_f64 * t26477 * t6993 + 4.0_f64 * t32766 * t3758 + t114264 - t120590 - t120594 - t120606 + t120607 + t120611 + t120612;
    t120613
}
