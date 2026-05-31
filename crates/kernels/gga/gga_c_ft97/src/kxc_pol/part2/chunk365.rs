//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 365/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk365<F: Float>(t103: F, t1841: F, t108: F, t1538: F, t1761: F, t1821: F, t1826: F, t1854: F, t1920: F, t438: F, t497: F, t88: F) -> (F, F) {
    let t1922 = t1841 * t103;
    let t1927 = -t108 * t1538 - t108 * t1761 - t1920 * t88 - F::cast_from(2.0_f64) * t438 * t497 - F::cast_from(2.0_f64) * t1821 - F::cast_from(4.0_f64) * t1826 + F::cast_from(4.0_f64) * t1854 + F::cast_from(2.0_f64) * t1922;
    (t1922, t1927)
}
