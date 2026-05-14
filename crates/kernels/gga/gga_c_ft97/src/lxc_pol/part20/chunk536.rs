//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 536/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk536<F: Float>(t1526: F, t2322: F, t9483: F, t2326: F, t342: F, t630: F, t2347: F, t240: F, t2349: F, t2360: F, t1934: F, t2321: F, t231: F, t2459: F, t2320: F, t2331: F, t2355: F, t2465: F, t343: F, t3806: F, t9482: F) -> (F, F, F, F, F, F, F) {
    let t9485 = t1526 * t9483 * t2322;
    let t9488 = t342 * t630 * t2326;
    let t9490 = t240 * t2347;
    let t9491 = t9490 * t2349;
    let t9498 = t240 * t2360;
    let t9499 = t9498 * t2349;
    let t9503 = t2321 * t1934;
    let t9507 = t231 * t2459;
    let t9511 = t2331 + t2465 + t9482 - t9485 / 18.0 - t9488 / 6.0 - t1526 * t3806 * t9491 / 9.0 - t1526 * t2320 * t2355 / 6.0 + t1526 * t2320 * t9499 / 6.0 - t1526 * t2320 * t9503 / 12.0 - t342 * t343 * t9507 / 4.0;
    (t9485, t9488, t9491, t9499, t9503, t9507, t9511)
}
