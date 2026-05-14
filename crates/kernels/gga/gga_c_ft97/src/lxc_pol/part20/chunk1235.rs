//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1235/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1235<F: Float>(t2409: F, t2665: F, t28746: F, t6317: F, t10683: F, t25165: F, t4162: F, t113341: F, t24981: F, t1882: F, t28813: F, t43912: F, t6318: F, t113190: F, t14678: F, t1486: F, t2399: F, t7071: F) -> (F, F, F, F, F, F, F) {
    let t113364 = t6317 * t2665 * t28746 * t2409;
    let t113368 = t6317 * t10683 * t25165 * t4162;
    let t113371 = t6317 * t24981 * t113341;
    let t113372 = t1882 * t28813;
    let t113373 = 4.0 / 9.0 * t113372;
    let t113374 = t43912 * t6318;
    let t113376 = t113190 * t113374 * t14678;
    let t113379 = t1486 * t2399 * t7071;
    (t113364, t113368, t113371, t113372, t113373, t113376, t113379)
}
