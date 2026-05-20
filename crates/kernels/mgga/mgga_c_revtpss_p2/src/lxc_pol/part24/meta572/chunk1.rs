//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1752/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1752<F: Float>(t12227: F, t20651: F, t6470: F, t1765: F, t82389: F, t20400: F, t6552: F, t12254: F, t141: F, t89863: F, t1145: F, t89845: F) -> (F, F, F, F, F) {
    let t90373 = F::cast_from(0.3103560775156404018e4_f64) * t12227 * t20651 * t6470;
    let t90375 = F::cast_from(0.23392894490538584828e1_f64) * t82389 * t1765;
    let t90377 = F::cast_from(0.35089341735807877242e1_f64) * t20400 * t6552;
    let t90379 = t141 * t12254 * t89863;
    let t90384 = t141 * t1145 * t89845;
    (t90373, t90375, t90377, t90379, t90384)
}
