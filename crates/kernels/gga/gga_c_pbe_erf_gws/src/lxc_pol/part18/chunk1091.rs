//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1091/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1091<F: Float>(t3912: F, t4473: F, t833: F, t2391: F, t3916: F, t3721: F, t938: F, t2409: F, t9296: F, t11889: F, t831: F, t1143: F, t2416: F) -> (F, F, F, F, F, F, F) {
    let t12198 = t3912 * t4473;
    let t12199 = t12198 * t833;
    let t12201 = t3916 * t2391;
    let t12204 = t3721 * t938;
    let t12206 = t2409 * t9296 * t12204;
    let t12210 = t2409 * t831 * t11889;
    let t12213 = t1143 * t2416;
    (t12198, t12199, t12201, t12204, t12206, t12210, t12213)
}
