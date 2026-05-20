//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1448/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1448<F: Float>(t41291: F, t41389: F, t41421: F, t41443: F, t964: F, t973: F, t981: F, t11591: F, t3026: F, t3034: F, t3030: F, t11465: F, t41225: F) -> (F, F, F, F, F, F) {
    let t41445 = t41291 + t41389 + t41421 + t41443;
    let t41449 = F::cast_from(0.5848223622634646207e0_f64) * t981 * t964 * t41445 * t973;
    let t41451 = F::cast_from(0.70178683471615754484e1_f64) * t11591 * t3026;
    let t41453 = F::cast_from(0.10389515463408878255e3_f64) * t11591 * t3034;
    let t41455 = F::cast_from(0.35089341735807877242e1_f64) * t11591 * t3030;
    let t41459 = F::cast_from(0.14035736694323150897e2_f64) * t981 * t11465 * t41225 * t973;
    (t41445, t41449, t41451, t41453, t41455, t41459)
}
