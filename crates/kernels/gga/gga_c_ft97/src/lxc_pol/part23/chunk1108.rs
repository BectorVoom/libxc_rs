//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1108/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1108<F: Float>(t1485: F, t1771: F, t6320: F, t6336: F, t8232: F, t1476: F, t9577: F, t9570: F, t2347: F, t6260: F, t2404: F, t2781: F, t2399: F, t6308: F, t6310: F, t1486: F, t6323: F) -> (F, F, F, F, F, F, F, F, F) {
    let t99314 = t1485 * t1771;
    let t99315 = t99314 * t6320;
    let t99317 = t8232 * t6336;
    let t99322 = t1476 * t9577;
    let t99352 = t1476 * t9570;
    let t99363 = t6260 * t2347;
    let t99391 = t2404 * t2781;
    let t99457 = t6308 * t2399 * t6310;
    let t99467 = t1486 * t2399 * t6323;
    (t99314, t99315, t99317, t99322, t99352, t99363, t99391, t99457, t99467)
}
