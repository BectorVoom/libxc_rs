//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1130/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1130<F: Float>(t624: F, t49: F, t606: F, t613: F, t6968: F, t72: F, t1927: F) -> (F, F, F, F) {
    let t6971 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t624;
    let t6972 = -F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t613 * t49 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6968 * t606 + t6971;
    let t6973 = t6972 * t72;
    let t6974 = t6973 * t1927;
    (t6971, t6972, t6973, t6974)
}
