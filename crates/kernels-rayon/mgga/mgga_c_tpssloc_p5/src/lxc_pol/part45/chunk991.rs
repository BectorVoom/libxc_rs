//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 991/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk991(t1880: f64, t24281: f64, t6553: f64, t6571: f64, t31420: f64, t6547: f64, t23171: f64, t23228: f64, t8547: f64, t31370: f64, t114866: f64, t6572: f64) -> (f64, f64, f64, f64, f64) {
    let t114937 = t1880 * t6553 * t6571 * t24281;
    let t114939 = t6547 * t31420;
    let t114943 = t23171 * t23228 * t8547;
    let t114944 = 0.82246703342411321824e-2_f64 * t114943;
    let t114945 = t6547 * t31370;
    let t114960 = t1880 * t114866 * t6572;
    (t114937, t114939, t114944, t114945, t114960)
}
