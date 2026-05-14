//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 830/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk830<F: Float>(t22914: F, t5504: F, t108: F, t5617: F, t379: F, t1564: F, t1307: F, t497: F, t1651: F, t5502: F, t1643: F, t7793: F, t5494: F, t92: F) -> (F, F, F, F, F, F, F, F) {
    let t22915 = t22914 * t5504;
    let t22917 = t5617 * t108;
    let t22918 = t22917 * t379;
    let t22919 = t1564 * t22918;
    let t22922 = t1307 * t497;
    let t22924 = t1564 * t22922 * t379;
    let t22928 = t1564 * t5502 * t1651;
    let t22932 = t7793 * t5502 * t1643;
    let t22935 = t5494 * t92;
    (t22915, t22917, t22919, t22922, t22924, t22928, t22932, t22935)
}
