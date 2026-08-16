//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta950 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3191;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3192;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta950(t17583: f64, t3172: f64, t3711: f64, t127: f64, t17693: f64, t17695: f64, t5268: f64, t17708: f64, t45779: f64, t13089: f64, t5391: f64, t13085: f64, t5381: f64, t1284: f64, t17306: f64, t3624: f64, t12916: f64, t17704: f64, t5340: f64, t12898: f64, t1804: f64, t12948: f64, t17529: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59386, t59391, t59401, t59404, t59406) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3191(t17583, t3172, t3711, t127, t17693, t17695, t5268, t17708, t45779, t13089, t5391, t13085, t5381);
        let (t59408, t59411, t59415, t59419, t59423) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3192(t13089, t5381, t1284, t17306, t3624, t12916, t17704, t5340, t12898, t1804, t12948, t17529);
    (t59386, t59391, t59401, t59404, t59406, t59408, t59411, t59415, t59419, t59423)
}
