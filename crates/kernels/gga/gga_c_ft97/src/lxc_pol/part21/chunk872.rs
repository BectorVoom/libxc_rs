//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 872/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk872<F: Float>(t614: F, t6616: F, t28: F, t376: F, t6621: F, t1349: F, t165: F, t6615: F, t379: F, t1969: F, t23405: F, t6584: F, t6617: F, t23997: F, t3483: F, t16658: F, t2: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t26560 = t6616 * t614;
    let t26561 = t28 * t26560;
    let t26564 = t376 * t6621;
    let t26565 = t1349 * t26564;
    let t26567 = t6615 * t165;
    let t26568 = t26567 * t379;
    let t26569 = t1969 * t26568;
    let t26572 = t23405 * t6584;
    let t26574 = t376 * t6617;
    let t26575 = t1349 * t26574;
    let t26577 = t23997 * t3483;
    let t26579 = t16658 * t2;
    (t26560, t26561, t26564, t26565, t26567, t26569, t26572, t26574, t26575, t26577, t26579)
}
