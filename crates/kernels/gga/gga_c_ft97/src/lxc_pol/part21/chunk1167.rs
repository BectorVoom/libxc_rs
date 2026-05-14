//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1167/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1167<F: Float>(t116532: F, t1564: F, t446: F, t116451: F, t25846: F, t28: F, t469: F, t5665: F, t965: F, t23009: F, t4505: F, t5617: F, t1882: F, t29698: F, t116312: F, t37305: F) -> (F, F, F, F, F, F, F) {
    let t116613 = t446 * t1564 * t116532;
    let t116616 = t446 * t1564 * t116451;
    let t116621 = t5665 * t28 * t469 * t25846 * t965;
    let t116626 = t23009 * t28 * t469 * t5617 * t4505;
    let t116628 = t1882 * t29698;
    let t116629 = 4.0 / 9.0 * t116628;
    let t116631 = t446 * t37305 * t116312;
    (t116613, t116616, t116621, t116626, t116628, t116629, t116631)
}
