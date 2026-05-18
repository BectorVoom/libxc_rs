//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 651/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk651<F: Float>(t454: F, t8232: F, t463: F, t480: F, t1637: F, t482: F, t89: F, t100: F, t8326: F, t104: F, t7943: F, t1786: F, t488: F) -> (F, F, F, F, F, F) {
    let t8485 = t8232 * t454;
    let t8506 = t463 * t480;
    let t8516 = t89 * t1637 * t482;
    let t8518 = t8326 * t100;
    let t8534 = F::new(28.0) / F::new(81.0) * t89 * t7943 * t104;
    let t8557 = t1786 * t488;
    (t8485, t8506, t8516, t8518, t8534, t8557)
}
