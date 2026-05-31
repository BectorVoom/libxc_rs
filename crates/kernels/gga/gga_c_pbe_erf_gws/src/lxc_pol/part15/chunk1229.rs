//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1229/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1229<F: Float>(t13776: F, t3038: F, t3975: F, t9504: F, t1113: F, t29117: F, t50949: F, t1114: F, t51717: F, t14138: F, t3093: F, t4386: F) -> (F, F, F, F, F) {
    let t52982 = t13776 * t3975 * t3038 * t9504;
    let t52986 = t13776 * t3975 * t1113 * t29117;
    let t52989 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t50949;
    let t52990 = t1114 * t51717;
    let t52991 = t52990 * t14138;
    let t52992 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t52991;
    let t52993 = t4386 * t3093;
    (t52982, t52986, t52989, t52992, t52993)
}
