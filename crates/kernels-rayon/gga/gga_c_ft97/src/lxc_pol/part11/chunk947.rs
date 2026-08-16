//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 947/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk947(t39673: f64, t2086: f64, t590: f64, t91: f64, t9243: f64, t37311: f64, t446: f64, t9327: f64, t1882: f64, t9075: f64, t9042: f64, t9034: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39674 = 280.0_f64 / 81.0_f64 * t39673;
    let t39677 = t91 * t2086 * t9243 * t590;
    let t39679 = t446 * t9327 * t37311;
    let t39681 = t1882 * t9075;
    let t39683 = t1882 * t9042;
    let t39685 = t1882 * t9034;
    (t39674, t39677, t39679, t39681, t39683, t39685)
}
