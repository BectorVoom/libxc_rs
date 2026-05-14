//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 848/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk848<F: Float>(t32194: F, t7932: F, t7963: F, t2132: F, t2139: F, t7885: F, t879: F, t3645: F, t611: F, t103: F, t2162: F, t104: F, t9081: F, t694: F, t9090: F, t9083: F, t96: F) -> (F, F, F, F, F, F, F) {
    let t32196 = t7963 * t7932 * t32194;
    let t32210 = 0.78062653693846795158e1 * t7885 * t2132 * t2139 * t879;
    let t32222 = 0.65854491829355115987e0 * t3645 * t611;
    let t32241 = t103 * t2162;
    let t33352 = t104 * t9081;
    let t33357 = 6.0 * t694 * t9090;
    let t33388 = 2.0 * t96 * t9083;
    (t32196, t32210, t32222, t32241, t33352, t33357, t33388)
}
