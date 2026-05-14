//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 876/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk876<F: Float>(t1882: F, t5970: F, t1391: F, t1647: F, t569: F, t1359: F, t1986: F, t167: F, t9432: F, t2180: F, t2179: F, t574: F, t1384: F, t9439: F, t144: F, t2185: F, t605: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t23484 = t1882 * t5970;
    let t23487 = t569 * t1391 * t1647;
    let t23490 = t1359 * t1986;
    let t23492 = t9432 * t167 * t23490;
    let t23495 = t1359 * t2180;
    let t23497 = t574 * t2179 * t23495;
    let t23500 = t1384 * t2180;
    let t23501 = t9439 * t23500;
    let t23502 = t144 * t23501;
    let t23505 = t1384 * t1986;
    let t23507 = t2185 * t605 * t23505;
    (t23484, t23487, t23490, t23492, t23495, t23497, t23500, t23501, t23502, t23505, t23507)
}
