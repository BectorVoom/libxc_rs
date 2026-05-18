//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 510/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk510<F: Float>(t1609: F, t2378: F, t2427: F, t6: F, t224: F, t1095: F, t2393: F, t51: F, t6032: F, t3771: F, t200: F, t709: F) -> (F, F, F, F, F, F, F, F) {
    let t13411 = t1609 * t2378;
    let t13442 = t2427 * t6;
    let t13443 = t224 * t13442;
    let t13469 = t2378 * t1095;
    let t13475 = t2393 * t1095;
    let t13519 = t6032 * t51;
    let t13520 = t3771 * t13519;
    let t13521 = t200 * t709;
    (t13411, t13442, t13443, t13469, t13475, t13519, t13520, t13521)
}
