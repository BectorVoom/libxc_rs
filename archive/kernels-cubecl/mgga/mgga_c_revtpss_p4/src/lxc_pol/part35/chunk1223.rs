//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1223/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1223<F: Float>(t106053: F, t106061: F, t106063: F, t106065: F, t113214: F, t113217: F, t95673: F, t95674: F, t95675: F, t95678: F, t95680: F, t99035: F, t99044: F, t99050: F) -> F {
    let t115698 = -F::cast_from(0.68598428988911579154e-3_f64) * t106053 - F::cast_from(0.68026775414003982662e-1_f64) * t99035 + F::cast_from(0.34299214494455789577e-3_f64) * t106061 + F::cast_from(0.12004725073059526352e-1_f64) * t106063 - F::cast_from(0.24009450146119052704e-1_f64) * t106065 + F::cast_from(0.12196800674228478774e-3_f64) * t99044 - t95673 + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t113214 - F::cast_from(35.0_f64) / F::cast_from(36.0_f64) * t99050 - t95674 + t95675 + t95678 - t113217 / F::cast_from(24.0_f64) - t95680;
    t115698
}
