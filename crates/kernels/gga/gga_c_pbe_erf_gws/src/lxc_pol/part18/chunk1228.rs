//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1228/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1228<F: Float>(t14392: F, t804: F, t1198: F, t321: F, t43260: F, t14380: F, t14835: F, t14817: F, t2053: F, t14387: F, t2429: F, t6926: F) -> (F, F, F, F, F, F, F) {
    let t52836 = F::cast_from(6.0_f64) * t804 * t14392;
    let t52853 = F::cast_from(4.0_f64) * t321 * t1198 * t43260;
    let t52855 = F::cast_from(6.0_f64) * t804 * t14380;
    let t52860 = F::cast_from(2.0_f64) * t321 * t14835;
    let t52861 = t14817 * t2053;
    let t52884 = F::cast_from(6.0_f64) * t804 * t14387;
    let t52887 = F::cast_from(12.0_f64) * t2429 * t1198 * t6926;
    (t52836, t52853, t52855, t52860, t52861, t52884, t52887)
}
