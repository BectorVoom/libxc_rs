//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1085/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1085<F: Float>(t14835: F, t321: F, t14817: F, t2053: F, t14387: F, t804: F, t1198: F, t2429: F, t6926: F, t4052: F, t8589: F, t829: F, t830: F, t13808: F, t14754: F, t14116: F, t3973: F) -> (F, F, F, F, F, F, F) {
    let t52860 = 2.0 * t321 * t14835;
    let t52861 = t14817 * t2053;
    let t52884 = 6.0 * t804 * t14387;
    let t52887 = 12.0 * t2429 * t1198 * t6926;
    let t52895 = t8589 * t4052;
    let t52897 = t829 * t830 * t52895;
    let t52901 = t13808 * t14754;
    let t52902 = 7.0 / 1152.0 * t52901;
    let t52906 = t3973 * t14116;
    (t52860, t52861, t52884, t52887, t52897, t52902, t52906)
}
