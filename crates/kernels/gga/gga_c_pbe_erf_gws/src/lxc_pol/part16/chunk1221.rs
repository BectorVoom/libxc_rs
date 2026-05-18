//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1221/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1221<F: Float>(t14116: F, t3973: F, t13776: F, t8886: F, t14733: F, t51588: F, t14469: F, t51581: F, t14423: F, t343: F, t361: F, t50998: F, t9505: F) -> (F, F, F, F, F) {
    let t52906 = t3973 * t14116;
    let t52908 = t13776 * t52906 * t8886;
    let t52910 = t14733 * t51588;
    let t52912 = t51581 * t14469;
    let t52915 = t361 * t14423 * t343;
    let t52917 = t50998 * t52915 * t9505;
    (t52908, t52910, t52912, t52915, t52917)
}
