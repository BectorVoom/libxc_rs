//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 729/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk729<F: Float>(t3103: F, t452: F, t499: F, t110: F, t11392: F, t1882: F, t3257: F, t1786: F, t11397: F, t463: F, t488: F, t1911: F, t2992: F) -> (F, F, F, F, F, F) {
    let t11459 = t452 * t499 * t3103;
    let t11463 = t452 * t110 * t11392;
    let t11467 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t3257;
    let t11468 = t1786 * t110;
    let t11469 = t11468 * t11397;
    let t11472 = t463 * t488;
    let t11473 = t2992 * t1911;
    (t11459, t11463, t11467, t11469, t11472, t11473)
}
