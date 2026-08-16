//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1151/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1151<F: Float>(t13796: F, t14601: F, t13859: F, t13972: F, t4146: F, t3166: F, t3990: F, t3991: F, t3989: F, t3979: F, t4150: F, t1178: F, t3097: F, t371: F) -> (F, F, F, F, F, F, F) {
    let t14602 = t13796 * t14601;
    let t14603 = t13859 * t14602;
    let t14605 = t13972 * t4146;
    let t14608 = t3990 * t3991 * t3166;
    let t14609 = t3989 * t14608;
    let t14611 = t3979 * t4150;
    let t14614 = t371 * t1178 * t3097;
    (t14602, t14603, t14605, t14608, t14609, t14611, t14614)
}
