//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta950 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3191;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3192;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta950<F: Float>(t17583: F, t3172: F, t3711: F, t127: F, t17693: F, t17695: F, t5268: F, t17708: F, t45779: F, t13089: F, t5391: F, t13085: F, t5381: F, t1284: F, t17306: F, t3624: F, t12916: F, t17704: F, t5340: F, t12898: F, t1804: F, t12948: F, t17529: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t59386, t59391, t59401, t59404, t59406) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3191::<F>(t17583, t3172, t3711, t127, t17693, t17695, t5268, t17708, t45779, t13089, t5391, t13085, t5381);
        let (t59408, t59411, t59415, t59419, t59423) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3192::<F>(t13089, t5381, t1284, t17306, t3624, t12916, t17704, t5340, t12898, t1804, t12948, t17529);
    (t59386, t59391, t59401, t59404, t59406, t59408, t59411, t59415, t59419, t59423)
}
