//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 925/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk925<F: Float>(t1058: F, t5842: F, t614: F, t6615: F, t2178: F, t6685: F, t27391: F, t604: F, t22511: F, t32772: F, t3392: F, t5818: F) -> (F, F, F, F, F, F) {
    let t104289 = t5842 * t1058;
    let t104364 = t6615 * t614;
    let t104462 = t6685 * t2178;
    let t104623 = t27391 * t604;
    let t104721 = t32772 * t22511;
    let t104722 = t3392 * t104721;
    let t104732 = t5818 * t104721;
    (t104289, t104364, t104462, t104623, t104722, t104732)
}
