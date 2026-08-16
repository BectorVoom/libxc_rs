//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1644/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1644<F: Float>(t6157: F, t6173: F, t11409: F, t15350: F, t15406: F, t19156: F, t23706: F, t23711: F, t2943: F, t2968: F, t2970: F, t41740: F, t41742: F, t6206: F, t6209: F, t64125: F, t88023: F, t88026: F, t88028: F, t88030: F, t88034: F, t88048: F, t88050: F, t88052: F, t88054: F, t954: F) -> (F, F) {
    let t88055 = t6157 * t6157;
    let t88068 = t6173 * t6173;
    let t88077 = F::cast_from(0.19964560303604640732e6_f64) * t41740 * t88055 * t41742 + F::cast_from(0.35089341735807877242e1_f64) * t19156 * t6206 + F::cast_from(0.10389515463408878255e3_f64) * t64125 * t6209 + t88023 - t88026 + F::cast_from(24.0_f64) * t15406 * t23706 - F::cast_from(24.0_f64) * t11409 * t88055 * t954 - F::cast_from(6.0_f64) * t2943 * t88068 * t954 + F::cast_from(0.96491876992155210402e2_f64) * t2968 * t88068 * t2970 + F::cast_from(0.14035736694323150897e2_f64) * t15350 * t23711 + t88028 + t88030 - t88034 - t88048 - t88050 - t88052 - t88054;
    (t88055, t88077)
}
