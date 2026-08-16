//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2794/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2794(t40297: f64, t4500: f64, t10069: f64, t14504: f64, t4423: f64, t860: f64, t1558: f64, t2760: f64, t10639: f64, t10666: f64, t14535: f64, t14663: f64, t2646: f64, t2815: f64, t39633: f64, t39635: f64, t39640: f64, t4366: f64, t4504: f64, t4514: f64, t4526: f64, t820: f64) -> (f64, f64, f64) {
    let t51371 = t40297 * t4500;
    let t51373 = t10069 * t14504;
    let t51374 = 0.21951497276451705329e-1_f64 * t51373;
    let t51375 = t860 * t4423;
    let t51380 = t2760 * t1558;
    let t51387 = t39633 + 0.91069445034239308175e-1_f64 * t39635 - 0.65854491829355115987e0_f64 * t820 * t4526 * t10666 - 0.19756347548806534796e1_f64 * t820 * t2815 * t14663 - 0.65854491829355115987e0_f64 * t820 * t4526 * t10639 - 0.29272321618148349057e-1_f64 * t51371 - t51374 + 0.79025390195226139182e1_f64 * t4504 * t51375 * t4366 - 0.34697458558045176417e-2_f64 * t39640 + 0.39512695097613069591e1_f64 * t4504 * t51380 * t4366 - 0.19756347548806534796e1_f64 * t4514 * t14535 * t2646;
    (t51375, t51380, t51387)
}
