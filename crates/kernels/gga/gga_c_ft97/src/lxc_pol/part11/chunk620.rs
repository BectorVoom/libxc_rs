//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 620/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk620<F: Float>(t2190: F, t379: F, t9144: F, t2142: F, t2157: F, t144: F, t1901: F, t446: F, t9090: F, t9094: F, t9097: F, t9100: F, t9104: F, t9106: F, t9109: F, t9112: F, t9118: F, t9124: F, t9129: F, t9136: F, t9141: F) -> (F, F, F, F, F) {
    let t9145 = t2190 * t379;
    let t9146 = t9144 * t9145;
    let t9149 = t2142 * t2157;
    let t9150 = t144 * t9149;
    let t9152 = -2.0 / 9.0 * t9090 + t1901 * t9094 / 3.0 - 2.0 / 9.0 * t9097 + 2.0 / 3.0 * t1901 * t9100 - t446 * t9104 + t9106 / 3.0 + 2.0 / 3.0 * t446 * t9109 - 2.0 / 9.0 * t9112 + 2.0 / 9.0 * t1901 * t9118 + 2.0 / 9.0 * t1901 * t9124 - 2.0 / 3.0 * t1901 * t9129 - 2.0 / 3.0 * t1901 * t9136 - 2.0 / 3.0 * t1901 * t9141 - 2.0 / 3.0 * t1901 * t9146 - t446 * t9150;
    (t9145, t9146, t9149, t9150, t9152)
}
