//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 694/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk694<F: Float>(t1902: F, t20214: F, t4431: F, t979: F, t1910: F, t1909: F, t15978: F, t15980: F, t16083: F, t16126: F, t1901: F, t20172: F, t20179: F, t20184: F, t20188: F, t20193: F, t20196: F, t20200: F, t20205: F, t20210: F, t446: F) -> (F, F, F, F, F) {
    let t20215 = t1902 * t20214;
    let t20218 = t4431 * t979;
    let t20219 = t1910 * t20218;
    let t20220 = t1909 * t20219;
    let t20223 = t15978 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t15980 - F::new(2.0) / F::new(3.0) * t1901 * t20172 - F::new(2.0) / F::new(9.0) * t16083 - t16126 / F::new(3.0) - F::new(2.0) * t446 * t20179 + F::new(2.0) * t446 * t20184 + F::new(2.0) * t446 * t20188 + t446 * t20193 - t446 * t20196 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t446 * t20200 + F::new(2.0) / F::new(9.0) * t1901 * t20205 + F::new(2.0) / F::new(9.0) * t1901 * t20210 + t1901 * t20215 / F::new(3.0) + t1901 * t20220 / F::new(3.0);
    (t20215, t20218, t20219, t20220, t20223)
}
