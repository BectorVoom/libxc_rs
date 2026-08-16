//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3740/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3740<F: Float>(t20809: F, t372: F, t12772: F, t21172: F, t5331: F, t44307: F, t68253: F, t68255: F, t68257: F, t68262: F, t68267: F, t68271: F, t68275: F, t68277: F, t68282: F, t68287: F, t68292: F) -> (F, F, F) {
    let t71112 = t372 * t20809;
    let t71117 = t5331 * t12772 * t21172;
    let t71134 = F::cast_from(0.33333333333333333334e-1_f64) * t68253 + F::cast_from(0.37037037037037037037e-2_f64) * t68255 - F::cast_from(0.24691358024691358024e-2_f64) * t68257 + t44307 - F::cast_from(0.61728395061728395061e-2_f64) * t68262 + F::cast_from(0.92592592592592592592e-2_f64) * t68267 + F::cast_from(0.2e0_f64) * t68271 + F::cast_from(0.33333333333333333334e-1_f64) * t68275 - F::cast_from(0.11111111111111111111e-1_f64) * t68277 - F::cast_from(0.11111111111111111111e-1_f64) * t68282 - F::cast_from(0.55555555555555555555e-2_f64) * t68287 - F::cast_from(0.33333333333333333333e-1_f64) * t68292;
    (t71112, t71117, t71134)
}
