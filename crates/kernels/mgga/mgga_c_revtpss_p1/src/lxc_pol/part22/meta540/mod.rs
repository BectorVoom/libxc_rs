//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta540 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2350;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2351;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta540<F: Float>(t15936: F, t17550: F, t1042: F, t3708: F, t5265: F, t13392: F, t5302: F, t1252: F, t1261: F, t12956: F, t17525: F, t17529: F, t17536: F, t17541: F, t17546: F, t17547: F, t3591: F, t3606: F, t3613: F, t3711: F, t5293: F, t5299: F, t1260: F, t5326: F) -> (F, F, F, F, F, F, F) {
        let (t17551, t17552, t17556, t17557, t17558, t17561) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2350::<F>(t15936, t17550, t1042, t3708, t5265, t13392, t5302, t1252, t1261, t12956, t17525, t17529, t17536, t17541, t17546, t17547, t3591, t3606, t3613, t3711, t5293, t5299);
        let t17569 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2351::<F>(t1260, t5326);
    (t17551, t17552, t17556, t17557, t17558, t17561, t17569)
}
