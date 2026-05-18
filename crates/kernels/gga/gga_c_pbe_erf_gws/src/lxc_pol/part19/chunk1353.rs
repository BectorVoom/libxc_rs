//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1353/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1353<F: Float>(t15406: F, t804: F, t15571: F, t321: F, t15397: F, t2429: F, t1205: F, t35889: F, t829: F, t830: F, t11348: F, t4083: F) -> (F, F, F, F, F) {
    let t57946 = t804 * t15406;
    let t57951 = t321 * t15571;
    let t57953 = t2429 * t15397;
    let t57956 = t35889 * t1205;
    let t57958 = t829 * t830 * t57956;
    let t57972 = t11348 * t4083;
    (t57946, t57951, t57953, t57958, t57972)
}
