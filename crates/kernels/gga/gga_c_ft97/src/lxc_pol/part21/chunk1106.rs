//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1106/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1106<F: Float>(t27337: F, t8392: F, t26854: F, t27017: F, t1882: F, t27265: F, t27289: F, t27199: F, t6636: F, t8232: F, t50235: F, t5942: F, t26988: F, t26842: F, t26876: F, t27008: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t106912 = 4.0 / 3.0 * t8392 * t27337;
    let t106914 = 2.0 / 27.0 * t8392 * t26854;
    let t106928 = 4.0 / 9.0 * t8392 * t27017;
    let t106934 = 2.0 / 9.0 * t1882 * t27265;
    let t106940 = 4.0 / 9.0 * t1882 * t27289;
    let t106957 = 2.0 / 9.0 * t1882 * t27199;
    let t106958 = t8232 * t6636;
    let t106981 = t50235 * t5942;
    let t107012 = 2.0 / 27.0 * t8392 * t26988;
    let t107019 = 2.0 / 9.0 * t1882 * t26842;
    let t107022 = 2.0 / 9.0 * t1882 * t26876;
    let t107024 = 2.0 / 27.0 * t8392 * t27008;
    (t106912, t106914, t106928, t106934, t106940, t106957, t106958, t106981, t107012, t107019, t107022, t107024)
}
