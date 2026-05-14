//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1086/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1086<F: Float>(t1178: F, t904: F, t14688: F, t2397: F, t13972: F, t14726: F, t13808: F, t14588: F, t13772: F, t3083: F, t14437: F, t2367: F, t1114: F, t51717: F, t14138: F, t3093: F, t4386: F) -> (F, F, F, F, F, F, F, F, F) {
    let t52926 = t904 * t1178;
    let t52930 = t14688 * t2397;
    let t52931 = 7.0 / 144.0 * t52930;
    let t52961 = t13972 * t14726;
    let t52962 = 7.0 / 2304.0 * t52961;
    let t52968 = t13808 * t14588;
    let t52969 = 7.0 / 1152.0 * t52968;
    let t52971 = 7.0 / 144.0 * t3083 * t13772;
    let t52973 = 7.0 / 144.0 * t2367 * t14437;
    let t52990 = t1114 * t51717;
    let t52991 = t52990 * t14138;
    let t52992 = 7.0 / 144.0 * t52991;
    let t52993 = t4386 * t3093;
    (t52926, t52931, t52962, t52969, t52971, t52973, t52990, t52992, t52993)
}
