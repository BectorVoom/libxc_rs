//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1224/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1224<F: Float>(t14733: F, t51588: F, t14469: F, t51581: F, t14423: F, t343: F, t361: F, t50998: F, t9505: F, t14673: F, t2397: F, t3165: F, t376: F) -> (F, F, F, F, F, F) {
    let t52910 = t14733 * t51588;
    let t52912 = t51581 * t14469;
    let t52915 = t361 * t14423 * t343;
    let t52917 = t50998 * t52915 * t9505;
    let t52919 = t14673 * t2397;
    let t52921 = t376 * t3165;
    (t52910, t52912, t52915, t52917, t52919, t52921)
}
