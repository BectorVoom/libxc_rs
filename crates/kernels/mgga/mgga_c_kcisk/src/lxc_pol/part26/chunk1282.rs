//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1282/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1282<F: Float>(t2721: F, t382: F, t114304: F, t9446: F, t113639: F, t33521: F, t9442: F, t6174: F, t9452: F, t1308: F, t14242: F, t2158: F, t13329: F, t394: F, t1333: F, t33498: F) -> (F, F, F, F, F, F, F, F) {
    let t114480 = t2721 * t382;
    let t114493 = 0.13888888888888888889e-1 * t9446 * t114304;
    let t114499 = 0.69444444444444444446e-2 * t9446 * t113639;
    let t114517 = 0.69444444444444444446e-2 * t33521 * t9442;
    let t114531 = t6174 * t9452;
    let t114580 = t14242 * t2158 * t1308;
    let t114585 = t13329 * t394;
    let t114596 = t1333 * t33498;
    (t114480, t114493, t114499, t114517, t114531, t114580, t114585, t114596)
}
