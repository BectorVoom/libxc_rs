//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1294/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1294(t114038: f64, t1338: f64, t31584: f64, t31560: f64, t6914: f64, t225: f64, t31573: f64, t31590: f64, t6883: f64, t22724: f64, t31594: f64, t2085: f64, t213: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t115465 = 119.0_f64 / 3456.0_f64 * t114038;
    let t115486 = t1338 * t31584;
    let t115508 = t6914 * t31560;
    let t115519 = t31573 * t225;
    let t115530 = t6883 * t31590;
    let t115539 = t22724 * t31594;
    let t115540 = 0.26044789391763585244e-1_f64 * t115539;
    let t115545 = t213 * t2085 * t225;
    (t115465, t115486, t115508, t115519, t115530, t115540, t115545)
}
