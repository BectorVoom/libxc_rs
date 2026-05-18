//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 970/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk970<F: Float>(t1000: F, t2784: F, t1827: F, t587: F, t1017: F, t2768: F, t7720: F, t3425: F, t562: F, t1821: F, t1820: F, t172: F, t3486: F) -> (F, F, F, F) {
    let t10956 = t1000 * t2784;
    let t10957 = t1827 * t10956;
    let t10959 = F::new(8.0) / F::new(45.0) * t587 * t10957;
    let t10960 = t2768 * t1017;
    let t10961 = t7720 * t10960;
    let t10963 = F::new(16.0) / F::new(45.0) * t587 * t10961;
    let t10964 = t3425 * t562;
    let t10965 = t1821 * t10964;
    let t10967 = F::new(16.0) / F::new(45.0) * t1820 * t10965;
    let t10968 = t172 * t3486;
    (t10959, t10963, t10967, t10968)
}
