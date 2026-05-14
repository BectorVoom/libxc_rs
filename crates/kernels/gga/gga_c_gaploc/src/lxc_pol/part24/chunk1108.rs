//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1108/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1108<F: Float>(t169: F, t2084: F, t2089: F, t32163: F, t32514: F, t7634: F, t8942: F, t10763: F, t7137: F, t21571: F, t3440: F, t10760: F, t7129: F, t29437: F, t29441: F, t29445: F, t29447: F, t29450: F, t29453: F, t29455: F, t29457: F, t32511: F) -> (F,) {
    let t32517 = 0.92286314761706691402e-1 * t32163 * t2084 * t169 * t2089 * t32514;
    let t32520 = 0.15381052460284448567e-1 * t32163 * t7634 * t8942;
    let t32522 = 0.6152420984113779427e-1 * t7137 * t10763;
    let t32524 = 0.23071578690426672851e-1 * t21571 * t3440;
    let t32526 = 0.46143157380853345702e-1 * t7129 * t10760;
    let t32527 = -t29437 - t29441 - t29445 - t29447 + t29450 + t29453 + t29455 - t29457 + t32511 + t32517 - t32520 - t32522 - t32524 - t32526;
    (t32527,)
}
