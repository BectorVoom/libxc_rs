//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2086/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2086<F: Float>(t22025: F, t543: F, t3992: F, t2661: F, t1370: F, t13779: F, t13781: F, t13797: F, t1410: F, t22038: F, t22041: F, t22044: F, t22048: F, t22052: F, t22057: F, t22059: F, t5671: F, t9735: F) -> (F, F, F, F) {
    let t22061 = t22025 * t543;
    let t22062 = t3992 * t22061;
    let t22063 = t2661 * t22062;
    let t22065 = -F::cast_from(0.15244095330869239812e-3_f64) * t13779 - F::cast_from(0.45351183609335988442e-1_f64) * t13781 + F::new(7.0) / F::new(144.0) * t22038 - t1370 * t22041 / F::new(48.0) - F::new(7.0) / F::new(48.0) * t22044 - t9735 - F::cast_from(0.17149607247227894789e-2_f64) * t5671 * t22048 + t13797 - F::cast_from(0.85748036236139473944e-3_f64) * t1410 * t22052 - F::cast_from(0.50820002809285328225e-4_f64) * t22057 - F::cast_from(0.20007875121765877254e-1_f64) * t22059 + F::cast_from(0.71456696863449561619e-5_f64) * t22063;
    (t22061, t22062, t22063, t22065)
}
