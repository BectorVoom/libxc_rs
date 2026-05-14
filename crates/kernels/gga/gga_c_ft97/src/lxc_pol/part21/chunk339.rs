//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 339/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk339<F: Float>(t1903: F, t3204: F, t1902: F, t379: F, t447: F, t986: F, t1848: F, t1883: F, t1887: F, t1888: F, t1890: F, t1901: F, t28: F, t3115: F, t3172: F, t3177: F, t3184: F, t3190: F, t3196: F, t3201: F, t446: F, t89: F) -> (F, F, F, F) {
    let t3205 = t1903 * t3204;
    let t3206 = t1902 * t3205;
    let t3210 = t447 * t986 * t379;
    let t3213 = t1901 * t3115 / 9.0 + t1883 / 27.0 + t89 * t28 * t3172 / 3.0 - t3177 / 9.0 - t1848 / 9.0 + t1887 + t1890 / 9.0 + t1888 / 9.0 + t1901 * t3184 / 9.0 + 2.0 / 9.0 * t1901 * t3190 - 2.0 / 27.0 * t1901 * t3196 + t1901 * t3201 / 9.0 + t1901 * t3206 / 9.0 - t446 * t3210 / 9.0;
    (t3205, t3206, t3210, t3213)
}
