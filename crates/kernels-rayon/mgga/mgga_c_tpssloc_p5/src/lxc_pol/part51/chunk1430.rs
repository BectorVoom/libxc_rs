//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1430/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1430(t22635: f64, t26331: f64, t31549: f64, t5308: f64, t1985: f64, t26193: f64, t31607: f64, t115519: f64, t120309: f64, t120312: f64, t120313: f64, t120316: f64, t120321: f64, t120324: f64, t1375: f64, t1843: f64, t2016: f64, t2091: f64, t22670: f64, t26471: f64, t33294: f64, t3882: f64, t3887: f64, t7937: f64, t93341: f64) -> f64 {
    let t122227 = t26331 * t22635 * t31549 * t5308;
    let t122235 = t1985 * t26193 * t31607;
    let t122240 = -0.49348022005446793095e-1_f64 * t122227 - t3882 * t33294 + 2.0_f64 * t1375 * t3887 * t2091 * t26471 + t120309 - t120312 + t120313 - t120316 - 0.82246703342411321825e-2_f64 * t122235 + t120321 - t93341 * t2016 - t120324 - t115519 * t1843 - t22670 * t7937;
    t122240
}
