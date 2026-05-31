//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1230/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1230<F: Float>(t52930: F, t13972: F, t14726: F, t13808: F, t14588: F, t13772: F, t3083: F, t14437: F, t2367: F, t1114: F, t51717: F, t14138: F) -> (F, F, F, F, F, F, F) {
    let t52931 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t52930;
    let t52961 = t13972 * t14726;
    let t52962 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t52961;
    let t52968 = t13808 * t14588;
    let t52969 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t52968;
    let t52971 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3083 * t13772;
    let t52973 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t2367 * t14437;
    let t52990 = t1114 * t51717;
    let t52991 = t52990 * t14138;
    (t52931, t52962, t52969, t52971, t52973, t52990, t52991)
}
