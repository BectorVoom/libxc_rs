//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2226/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2226(t52: f64, t10913: f64, t12606: f64, t12874: f64, t12877: f64, t1409: f64, t2244: f64, t2250: f64, t2440: f64, t3966: f64, t40647: f64, t4087: f64, t45872: f64, t607: f64, t76: f64, t9258: f64, t9288: f64, t9438: f64, zeta_threshold: f64) -> f64 {
    let t150 = t52 <= zeta_threshold;
    let t46190 = piecewise3(t150, 0.0_f64, 40.0_f64 / 81.0_f64 * t40647 * t1409 * t9288 + 8.0_f64 / 9.0_f64 * t9438 * t3966 * t2244 + 8.0_f64 / 9.0_f64 * t12874 * t10913 + 4.0_f64 / 3.0_f64 * t2440 * t12606 * t607 + 4.0_f64 / 3.0_f64 * t12877 * t2250 + 4.0_f64 / 9.0_f64 * t4087 * t9258 - 4.0_f64 / 3.0_f64 * t76 * t45872);
    t46190
}
