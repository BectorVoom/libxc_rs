//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 620/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk620<F: Float>(t100: F, t370: F, t1882: F, t3263: F, t3240: F, t3235: F, t8232: F, t981: F, t110: F, t8326: F, t1780: F, t488: F, t3172: F, t376: F, t89: F, t1637: F, t973: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11490 = t370 * t100;
    let t11535 = 2.0 / 9.0 * t1882 * t3263;
    let t11537 = 2.0 / 9.0 * t1882 * t3240;
    let t11549 = 2.0 / 9.0 * t1882 * t3235;
    let t11550 = t8232 * t981;
    let t11552 = t8326 * t110;
    let t11556 = t1780 * t488;
    let t11567 = 2.0 / 9.0 * t89 * t376 * t3172;
    let t11578 = t89 * t1637 * t973;
    (t11490, t11535, t11537, t11549, t11550, t11552, t11556, t11567, t11578)
}
