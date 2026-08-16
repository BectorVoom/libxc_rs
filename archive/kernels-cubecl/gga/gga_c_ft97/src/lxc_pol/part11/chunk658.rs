//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 658/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk658<F: Float>(t2190: F, t379: F, t9144: F, t2142: F, t2157: F, t144: F, t1901: F, t446: F, t9090: F, t9094: F, t9097: F, t9100: F, t9104: F, t9106: F, t9109: F, t9112: F, t9118: F, t9124: F, t9129: F, t9136: F, t9141: F) -> (F, F, F, F, F) {
    let t9145 = t2190 * t379;
    let t9146 = t9144 * t9145;
    let t9149 = t2142 * t2157;
    let t9150 = t144 * t9149;
    let t9152 = -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t9090 + t1901 * t9094 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t9097 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t9100 - t446 * t9104 + t9106 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t9109 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t9112 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t9118 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t9124 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t9129 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t9136 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t9141 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t9146 - t446 * t9150;
    (t9145, t9146, t9149, t9150, t9152)
}
