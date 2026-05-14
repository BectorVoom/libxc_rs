//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 644/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk644<F: Float>(t1882: F, t6636: F, t3429: F, t5916: F, t9144: F, t13208: F, t27068: F, t13212: F, t27073: F, t1901: F, t23468: F, t23484: F, t27199: F, t27203: F, t27205: F, t27208: F, t27212: F, t27217: F, t27222: F, t446: F) -> (F, F) {
    let t27226 = t1882 * t6636;
    let t27228 = t5916 * t3429;
    let t27229 = t9144 * t27228;
    let t27232 = t13208 * t27068;
    let t27235 = t13212 * t27073;
    let t27238 = -t446 * t27199 / 3.0 - t23468 / 27.0 + t27203 / 9.0 + t27205 / 9.0 - t1901 * t27208 / 9.0 - 2.0 / 9.0 * t1901 * t27212 - 2.0 / 9.0 * t1901 * t27217 + 2.0 / 27.0 * t1901 * t27222 + t23484 / 9.0 + t27226 / 9.0 - t1901 * t27229 / 9.0 - 2.0 / 9.0 * t1901 * t27232 + 2.0 / 27.0 * t1901 * t27235;
    (t27228, t27238)
}
