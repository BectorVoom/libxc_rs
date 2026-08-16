//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 354/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk354<F: Float>(t1984: F, t2: F, t1956: F, t376: F, t599: F, t89: F, t597: F, t604: F, t161: F, t1637: F, t1882: F, t576: F) -> (F, F, F, F, F, F, F) {
    let t2112 = t1984 * t2;
    let t2124 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1956;
    let t2140 = t89 * t376 * t599;
    let t2142 = t597 * t604;
    let t2149 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1956;
    let t2164 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t89 * t1637 * t161;
    let t2165 = t1882 * t576;
    (t2112, t2124, t2140, t2142, t2149, t2164, t2165)
}
