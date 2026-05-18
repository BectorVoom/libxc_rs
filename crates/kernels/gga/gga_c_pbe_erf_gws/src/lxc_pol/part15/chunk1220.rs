//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1220/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1220<F: Float>(t14392: F, t804: F, t1167: F, t2074: F, t1172: F, t2182: F, t1105: F, t2423: F, t1198: F, t321: F, t43260: F, t14380: F) -> (F, F, F, F, F, F) {
    let t52836 = F::new(6.0) * t804 * t14392;
    let t52837 = t1167 * t2074;
    let t52841 = t1172 * t2182;
    let t52847 = t1105 * t2423;
    let t52853 = F::new(4.0) * t321 * t1198 * t43260;
    let t52855 = F::new(6.0) * t804 * t14380;
    (t52836, t52837, t52841, t52847, t52853, t52855)
}
