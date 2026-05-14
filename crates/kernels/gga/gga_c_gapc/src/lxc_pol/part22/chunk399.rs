//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 399/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk399<F: Float>(t2053: F, t2058: F, t2059: F, t2078: F, t2141: F, t2150: F, t257: F, t260: F, t266: F, t414: F, t738: F, t748: F, t751: F, t758: F, t276: F) -> (F, F) {
    let t2152 = 0.21272952746160294864e-2 * t414 * t257 + 0.42545905492320589728e-2 * t2053 * t748 + 0.63818858238480884592e-2 * t2058 * t2059 - 0.21272952746160294864e-2 * t738 * t2078 - t2141 * t266 - 2.0 * t751 * t758 - t260 * t2150;
    let t2153 = t2152 * t276;
    (t2152, t2153)
}
