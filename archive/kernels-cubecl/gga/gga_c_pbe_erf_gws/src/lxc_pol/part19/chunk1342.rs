//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1342/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1342<F: Float>(t3912: F, t50887: F, t14138: F, t2409: F, t35890: F, t3965: F, t12243: F, t14121: F, t1113: F, t1161: F, t13781: F, t2271: F, t3972: F) -> (F, F, F, F) {
    let t57604 = t3912 * t50887;
    let t57605 = t57604 * t14138;
    let t57608 = t3965 * t2409 * t35890;
    let t57614 = t14121 * t12243;
    let t57626 = t3972 * t13781 * t1113 * t2271 * t1161;
    (t57605, t57608, t57614, t57626)
}
