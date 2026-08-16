//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 717/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk717<F: Float>(t2552: F, t164: F, t172: F) -> (F, F, F, F) {
    let t2553 = F::cast_from(1.0_f64) / t2552;
    let t2554 = t164 * t2553;
    let t2555 = t172 * t172;
    let t2556 = F::cast_from(1.0_f64) / t2555;
    (t2553, t2554, t2555, t2556)
}
