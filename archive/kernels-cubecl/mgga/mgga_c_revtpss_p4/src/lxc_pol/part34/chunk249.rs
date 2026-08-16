//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 249/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk249<F: Float>(t902: F, t139: F, t221: F, t346: F, t345: F, t220: F, t344: F, t44: F) -> (F, F, F, F, F) {
    let t997 = F::cast_from(0.14816666666666666667e-1_f64) * t902;
    let t1007 = t221 * t139 * t346;
    let t1009 = t345 * t1007 / F::cast_from(288.0_f64);
    let t1010 = t344 * t220;
    let t1011 = t44 * t1010;
    (t997, t1007, t1009, t1010, t1011)
}
