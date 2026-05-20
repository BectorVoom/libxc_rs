//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3162/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3162<F: Float>(t24803: F, t3625: F, t44425: F, t12787: F, t17448: F, t17605: F, t17729: F, t20265: F, t21020: F, t21040: F, t21157: F, t21161: F, t24240: F, t3626: F, t5402: F, t5405: F, t6638: F, t70039: F, t70044: F, t70819: F, t70944: F, t82481: F) -> F {
    let t83067 = t3625 * t44425 * t24803;
    let t83081 = -F::cast_from(0.85748036236139473944e-3_f64) * t17448 * t21161 - F::cast_from(0.85748036236139473944e-3_f64) * t3625 * t3626 * t24240 * t5405 + F::cast_from(0.45732285992607719436e-2_f64) * t17605 * t21161 - F::cast_from(0.42874018118069736972e-3_f64) * t70819 * t5402 - F::cast_from(0.42874018118069736972e-3_f64) * t3625 * t3626 * t70944 * t6638 + F::cast_from(0.47637797908966374413e-3_f64) * t83067 - F::cast_from(0.42874018118069736972e-3_f64) * t3625 * t3626 * t21040 * t21020 - F::cast_from(0.7145669686344956162e-3_f64) * t17729 * t12787 * t20265 * t82481 - F::cast_from(0.57165357490759649295e-3_f64) * t70039 - F::cast_from(0.57165357490759649295e-3_f64) * t70044 - F::cast_from(0.42874018118069736972e-3_f64) * t17448 * t21157;
    t83081
}
