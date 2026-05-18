//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 687/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk687<F: Float>(t3190: F, t8392: F, t3115: F, t1882: F, t3257: F, t110: F, t1786: F, t463: F, t488: F, t100: F, t370: F, t3263: F) -> (F, F, F, F, F, F, F) {
    let t11436 = F::new(4.0) / F::new(27.0) * t8392 * t3190;
    let t11448 = F::new(2.0) / F::new(27.0) * t8392 * t3115;
    let t11467 = F::new(2.0) / F::new(9.0) * t1882 * t3257;
    let t11468 = t1786 * t110;
    let t11472 = t463 * t488;
    let t11490 = t370 * t100;
    let t11535 = F::new(2.0) / F::new(9.0) * t1882 * t3263;
    (t11436, t11448, t11467, t11468, t11472, t11490, t11535)
}
