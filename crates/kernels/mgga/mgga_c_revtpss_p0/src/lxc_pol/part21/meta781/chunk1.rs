//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2794/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2794<F: Float>(t40297: F, t4500: F, t10069: F, t14504: F, t4423: F, t860: F, t1558: F, t2760: F, t10639: F, t10666: F, t14535: F, t14663: F, t2646: F, t2815: F, t39633: F, t39635: F, t39640: F, t4366: F, t4504: F, t4514: F, t4526: F, t820: F) -> (F, F, F) {
    let t51371 = t40297 * t4500;
    let t51373 = t10069 * t14504;
    let t51374 = F::cast_from(0.21951497276451705329e-1_f64) * t51373;
    let t51375 = t860 * t4423;
    let t51380 = t2760 * t1558;
    let t51387 = t39633 + F::cast_from(0.91069445034239308175e-1_f64) * t39635 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t4526 * t10666 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t2815 * t14663 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t4526 * t10639 - F::cast_from(0.29272321618148349057e-1_f64) * t51371 - t51374 + F::cast_from(0.79025390195226139182e1_f64) * t4504 * t51375 * t4366 - F::cast_from(0.34697458558045176417e-2_f64) * t39640 + F::cast_from(0.39512695097613069591e1_f64) * t4504 * t51380 * t4366 - F::cast_from(0.19756347548806534796e1_f64) * t4514 * t14535 * t2646;
    (t51375, t51380, t51387)
}
