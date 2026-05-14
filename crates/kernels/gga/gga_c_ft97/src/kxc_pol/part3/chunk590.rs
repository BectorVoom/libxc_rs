//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 590/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk590<F: Float>(t466: F, t8282: F, t2: F, t8216: F, t3051: F, t94: F, t1771: F, t471: F, t1554: F, t369: F, t631: F, t637: F, t7242: F, t96: F, t1786: F, t480: F) -> (F, F, F, F, F, F, F, F) {
    let t8283 = t8282 * t466;
    let t8291 = t8216 * t2;
    let t8301 = 28.0 / 27.0 * t3051 * t94;
    let t8302 = t1771 * t471;
    let t8326 = t1554 * t369;
    let t8327 = t8326 * t2;
    let t8345 = 1.0 / t96 / t631 / t637 / t369 / t7242 / 4.0;
    let t8372 = t1786 * t480;
    (t8283, t8291, t8301, t8302, t8326, t8327, t8345, t8372)
}
