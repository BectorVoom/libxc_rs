//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1148/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1148<F: Float>(t25893: F, t25894: F, t25899: F, t452: F, t100178: F, t15959: F, t1901: F, t2: F, t29569: F, t1564: F, t379: F, t5674: F, t16150: F, t93416: F, t100374: F, t16169: F, t22986: F) -> (F, F, F, F, F, F) {
    let t116302 = t25893 * t452 * t25899 * t25894;
    let t116305 = t1901 * t100178 * t15959;
    let t116307 = t2 * t29569;
    let t116310 = t5674 * t1564 * t116307 * t379;
    let t116312 = t93416 * t16150;
    let t116314 = t5674 * t100374 * t116312;
    let t116316 = t22986 * t16169;
    (t116302, t116305, t116310, t116312, t116314, t116316)
}
