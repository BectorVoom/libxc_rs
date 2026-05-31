//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 547/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk547<F: Float>(t221: F, t346: F, t696: F, t345: F, t2270: F, t344: F, t1003: F, t1007: F, t360: F, t365: F, t1038: F, t72: F) -> (F, F, F, F, F, F) {
    let t3080 = t221 * t696 * t346;
    let t3082 = t345 * t3080 / F::cast_from(432.0_f64);
    let t3083 = t2270 * t344;
    let t3086 = t1003 * t1007;
    let t3088 = t360 * t365;
    let t3089 = t1038 * t72;
    (t3080, t3082, t3083, t3086, t3088, t3089)
}
