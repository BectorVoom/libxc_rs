//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1302/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1302<F: Float>(t13776: F, t36889: F, t3975: F, t1113: F, t2417: F, t3972: F, t51548: F, t824: F, t13781: F, t13782: F, t3038: F, t13792: F, t8716: F) -> (F, F, F, F) {
    let t54697 = t13776 * t3975 * t36889;
    let t54702 = t3972 * t51548 * t1113 * t824 * t2417;
    let t54707 = t3972 * t13781 * t3038 * t13782;
    let t54714 = t13792 * t8716;
    (t54697, t54702, t54707, t54714)
}
