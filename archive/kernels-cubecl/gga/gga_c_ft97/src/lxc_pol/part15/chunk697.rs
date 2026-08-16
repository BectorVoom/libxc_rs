//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 697/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk697<F: Float>(t4458: F, t447: F, t986: F, t110: F, t20045: F, t20023: F, t8577: F, t4551: F, t942: F, t1852: F, t452: F, t4589: F, t979: F) -> (F, F, F, F, F, F) {
    let t20248 = t447 * t986 * t4458;
    let t20256 = t447 * t110 * t20045;
    let t20260 = t8577 * t110 * t20023;
    let t20263 = t4551 * t942;
    let t20265 = t452 * t1852 * t20263;
    let t20268 = t979 * t4589;
    (t20248, t20256, t20260, t20263, t20265, t20268)
}
