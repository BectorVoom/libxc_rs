//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 840/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk840<F: Float>(t33561: F, t342: F, t630: F, t1403: F, t24499: F, t24220: F, t7437: F, t33583: F, t681: F, t2399: F, t7486: F, t7442: F, t33568: F, t5999: F, t140768: F, t141200: F) -> (F, F, F, F, F, F, F, F, F) {
    let t141491 = t342 * t630 * t33561;
    let t141509 = t1403 * t24499;
    let t141524 = t7437 * t24220;
    let t141527 = t1403 * t681 * t33583;
    let t141543 = 2.0 / 27.0 * t1403 * t2399 * t7486;
    let t141552 = 4.0 / 27.0 * t1403 * t2399 * t7442;
    let t141560 = t33568 * t5999;
    let t141577 = 2.0 / 27.0 * t140768;
    let t141606 = 8.0 / 27.0 * t141200;
    (t141491, t141509, t141524, t141527, t141543, t141552, t141560, t141577, t141606)
}
