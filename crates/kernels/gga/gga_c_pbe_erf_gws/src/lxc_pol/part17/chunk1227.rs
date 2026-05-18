//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1227/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1227<F: Float>(t13859: F, t52926: F, t9284: F, t13972: F, t14726: F, t13808: F, t14588: F, t13772: F, t3083: F, t14437: F, t2367: F, t1113: F, t29103: F, t3972: F, t3975: F) -> (F, F, F, F, F, F) {
    let t52959 = t13859 * t52926 * t9284;
    let t52961 = t13972 * t14726;
    let t52962 = F::new(7.0) / F::new(2304.0) * t52961;
    let t52968 = t13808 * t14588;
    let t52969 = F::new(7.0) / F::new(1152.0) * t52968;
    let t52971 = F::new(7.0) / F::new(144.0) * t3083 * t13772;
    let t52973 = F::new(7.0) / F::new(144.0) * t2367 * t14437;
    let t52976 = t3972 * t3975 * t1113 * t29103;
    (t52959, t52962, t52969, t52971, t52973, t52976)
}
