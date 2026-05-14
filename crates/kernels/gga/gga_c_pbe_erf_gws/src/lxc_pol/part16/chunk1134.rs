//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1134/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1134<F: Float>(t2409: F, t26933: F, t3959: F, t13953: F, t3070: F, t4141: F, t50998: F, t9521: F, t1113: F, t13776: F, t20898: F, t3975: F, t36889: F, t2417: F, t3972: F, t51548: F, t824: F) -> (F, F, F, F, F, F) {
    let t54675 = t3959 * t2409 * t26933;
    let t54681 = t13953 * t3070;
    let t54690 = t50998 * t4141 * t9521;
    let t54694 = t13776 * t3975 * t1113 * t20898;
    let t54697 = t13776 * t3975 * t36889;
    let t54702 = t3972 * t51548 * t1113 * t824 * t2417;
    (t54675, t54681, t54690, t54694, t54697, t54702)
}
