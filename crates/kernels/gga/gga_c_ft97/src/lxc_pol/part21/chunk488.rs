//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 488/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk488<F: Float>(t363: F, t5916: F, t1969: F, t446: F, t558: F, t5778: F, t28: F, t89: F, t526: F, t5842: F, t27: F, t5894: F, t5898: F, t5903: F, t5907: F, t5911: F, t5915: F) -> (F, F, F, F, F, F, F) {
    let t5917 = t5916 * t363;
    let t5918 = t1969 * t5917;
    let t5919 = t446 * t5918;
    let t5921 = t5778 * t558;
    let t5922 = t28 * t5921;
    let t5923 = t89 * t5922;
    let t5925 = t526 * t5842;
    let t5927 = t89 * t27 * t5925;
    let t5929 = t5894 / 12.0 + t5898 + t5903 / 18.0 + t5907 / 3.0 - t5911 / 6.0 + t5915 + t5919 / 9.0 + 2.0 / 3.0 * t5923 - t5927 / 3.0;
    (t5918, t5919, t5921, t5923, t5925, t5927, t5929)
}
