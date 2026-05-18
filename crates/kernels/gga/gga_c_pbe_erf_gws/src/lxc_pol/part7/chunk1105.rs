//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1105/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1105<F: Float>(t2074: F, t353: F, t4386: F, t4387: F, t2365: F, t56: F, t2118: F, t822: F, t4484: F, t2395: F, t6161: F, t829: F, t830: F) -> (F, F, F, F, F) {
    let t19772 = t4386 * t353 * t4387 * t2074;
    let t19775 = t2365 * t56;
    let t19776 = t2118 * t19775;
    let t19777 = t822 * t19776;
    let t19778 = t19777 * t4484;
    let t19791 = t829 * t830 * t2395 * t6161;
    (t19772, t19775, t19777, t19778, t19791)
}
