//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1190/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1190<F: Float>(t11449: F, t11452: F, t190: F, t424: F, t11519: F, t34656: F, t11597: F, t9304: F, t9308: F, t20768: F, t34363: F, t11495: F, t1717: F) -> (F, F, F, F, F) {
    let t34739 = t424 * t190 * t11449 * t11452;
    let t34742 = t34656 * t11519;
    let t34745 = t9304 * t11597 * t9308;
    let t34747 = t34363 * t20768;
    let t34749 = t11495 * t1717;
    (t34739, t34742, t34745, t34747, t34749)
}
