//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1015/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1015<F: Float>(t1882: F, t24679: F, t24675: F, t8392: F, t24601: F, t24579: F, t24652: F, t681: F, t89: F, t1443: F, t9802: F, t24775: F, t24608: F, t6187: F, t668: F, t24726: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t97472 = t1882 * t24679;
    let t97488 = t8392 * t24675;
    let t97490 = t8392 * t24601;
    let t97492 = t8392 * t24579;
    let t97517 = t89 * t681 * t24652;
    let t97522 = t9802 * t1443;
    let t97526 = t1882 * t24775;
    let t97528 = t1882 * t24608;
    let t97537 = t6187 * t668;
    let t97559 = t8392 * t24726;
    (t97472, t97488, t97490, t97492, t97517, t97522, t97526, t97528, t97537, t97559)
}
