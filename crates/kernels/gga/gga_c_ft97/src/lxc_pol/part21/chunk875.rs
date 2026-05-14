//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 875/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk875<F: Float>(t26590: F, t609: F, t1053: F, t23478: F, t1023: F, t1349: F, t1362: F, t1389: F, t23406: F, t24074: F, t26561: F, t26565: F, t26569: F, t26572: F, t26575: F, t26577: F, t26581: F, t26584: F, t3313: F, t3414: F, t5772: F, t5973: F) -> (F, F, F) {
    let t26591 = t26590 * t609;
    let t26593 = t23478 * t1053;
    let t26595 = t23406 / 54.0 + t1349 * t26561 / 6.0 - t26565 / 18.0 - t5772 * t26569 / 18.0 + t26572 / 54.0 - t26575 / 18.0 + 4.0 * t26577 + t26581 * t1362 / 6.0 - 2.0 * t26584 + t24074 / 9.0 - t1023 * t5973 - t3313 * t1389 - t3414 * t1389 - 2.0 * t26591 - 2.0 * t26593;
    (t26591, t26593, t26595)
}
