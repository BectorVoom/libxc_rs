//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 724/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk724<F: Float>(t32032: F, t356: F, t461: F, t5700: F, t342: F, t630: F, t7155: F, t5617: F, t72: F, t1286: F, t1526: F, t1527: F, t2: F, t32026: F, t32031: F, t343: F, t5692: F, t5697: F, t7151: F, t7152: F) -> (F, F, F, F, F) {
    let t32033 = t356 * t32032;
    let t32038 = t461 * t5700;
    let t32043 = t342 * t630 * t7155 / F::cast_from(12.0_f64);
    let t32047 = t72 * t5617;
    let t32052 = (-t32026 * t7152 / F::cast_from(6.0_f64) + t32031 + t1286 * t32033 / F::cast_from(18.0_f64) + t1286 * t5697 / F::cast_from(3.0_f64) - t7151 * t32038 / F::cast_from(6.0_f64) - t32043 - t1526 * t1527 * t5692 / F::cast_from(12.0_f64) - t342 * t343 * t32047 / F::cast_from(4.0_f64)) * t2;
    (t32033, t32038, t32043, t32047, t32052)
}
