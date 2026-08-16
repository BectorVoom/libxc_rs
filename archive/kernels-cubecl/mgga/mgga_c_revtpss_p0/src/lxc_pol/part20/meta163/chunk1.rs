//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 876/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk876<F: Float>(t121: F, t131: F, t141: F, t22: F, t2456: F, t624: F) -> (F, F, F) {
    let t9282 = F::cast_from(1.0_f64) / t131 / t141 * t121 / F::cast_from(4.0_f64);
    let t9283 = t9282 * t22;
    let t9285 = t2456 * t624;
    (t9282, t9283, t9285)
}
