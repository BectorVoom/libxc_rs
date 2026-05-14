//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 998/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk998<F: Float>(t1403: F, t2399: F, t6063: F, t6067: F, t173: F, t24277: F, t24276: F, t24279: F, t2393: F, t420: F, t703: F, t3789: F, t41547: F, t6: F, t8: F, t24324: F, t24325: F, t24330: F) -> (F, F, F, F, F, F, F) {
    let t96397 = t1403 * t2399 * t6063;
    let t96400 = t1403 * t2399 * t6067;
    let t96419 = t173 * t24277;
    let t96421 = t24276 * t96419 * t24279;
    let t96424 = t420 * t703 * t2393;
    let t96442 = t3789 * t41547 * t6 * t8;
    let t96448 = t24324 * t24330 * t24325;
    (t96397, t96400, t96419, t96421, t96424, t96442, t96448)
}
