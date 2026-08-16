//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1340/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1340<F: Float>(t105814: F, t109339: F, t109345: F, t109348: F, t109349: F, t109351: F, t114826: F, t114883: F, t1458: F, t1914: F, t1921: F, t2038: F, t2045: F, t25049: F, t25072: F, t3: F, t30161: F, t30197: F, t575: F, t6937: F, t6951: F, t7940: F, t7956: F) -> F {
    let tv4rho3sigma9 = t114826 * t3 * t575 + t114883 * t1458 + F::cast_from(3.0_f64) * t1914 * t30197 + F::cast_from(3.0_f64) * t1921 * t30161 + t2038 * t25072 + t2045 * t25049 + F::cast_from(3.0_f64) * t6937 * t7956 + F::cast_from(3.0_f64) * t6951 * t7940 + F::cast_from(3.0_f64) * t105814 + F::cast_from(6.0_f64) * t109339 + F::cast_from(3.0_f64) * t109345 + F::cast_from(3.0_f64) * t109348 + F::cast_from(6.0_f64) * t109349 + F::cast_from(3.0_f64) * t109351;
    tv4rho3sigma9
}
