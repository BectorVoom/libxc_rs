//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2411/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2411<F: Float>(t11006: F, t256: F, t225: F, t252: F, t2769: F, t2782: F, t2828: F, t886: F, t2441: F, t39515: F, t10504: F, t138: F, t9302: F) -> (F, F, F, F) {
    let t41077 = F::cast_from(1.0_f64) / t11006 / t256;
    let t41078 = t225 * t41077;
    let t41092 = t2782 * t252 * t2769 * t886 * t2828;
    let t41095 = F::cast_from(0.11564373972601816912e-1_f64) * t39515 * t2441;
    let t41098 = t10504 * t138 * t9302 * t886;
    (t41078, t41092, t41095, t41098)
}
