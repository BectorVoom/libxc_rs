//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta932 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3161;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3162;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta932<F: Float>(t13014: F, t5373: F, t12998: F, t1222: F, t140: F, t17404: F, t12941: F, t5293: F, t5274: F, t1263: F, t16750: F, t17547: F, t3704: F, t17609: F, t12901: F, t17525: F, t1261: F, t17551: F, t3172: F, t3625: F, t44250: F, t5406: F, t12773: F, t17448: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t57290, t57292, t57295, t57297, t57299, t57303, t57314) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3161::<F>(t13014, t5373, t12998, t1222, t140, t17404, t12941, t5293, t5274, t1263, t16750, t17547, t3704);
        let (t57316, t57318, t57321, t57331, t57333) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3162::<F>(t17609, t3704, t12901, t17525, t1261, t17551, t3172, t3625, t44250, t5406, t12773, t17448);
    (t57290, t57292, t57295, t57297, t57299, t57303, t57314, t57316, t57318, t57321, t57331, t57333)
}
