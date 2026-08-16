//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta335 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1257;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1258;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta335(t12211: f64, t13206: f64, t1310: f64, t2371: f64, t10192: f64, t10194: f64, t10260: f64, t10263: f64, t10415: f64, t10416: f64, t10426: f64, t118: f64, t1315: f64, t1453: f64, t2320: f64, t2322: f64, t2328: f64, t2331: f64, t2372: f64, t3813: f64, t3821: f64, t4151: f64, t4254: f64, t508: f64, t511: f64, t569: f64, t649: f64, t651: f64, t671: f64, t3: f64, t2327: f64, t670: f64, t116: f64, t10259: f64, t117: f64, t1459: f64, t1461: f64, t4158: f64, t4162: f64, t4165: f64, t572: f64, t573: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13207, t13216, t13225) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1257(t12211, t13206, t1310, t2371, t10192, t10194, t10260, t10263, t10415, t10416, t10426, t118, t1315, t1453, t2320, t2322, t2328, t2331, t2372, t3813, t3821, t4151, t4254, t508, t511, t569, t649, t651, t671);
        let (t13226, t13232, t13240, t13243, t13244, t13247, t13250) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1258(t13225, t3, t2327, t670, t116, t2371, t10259, t117, t1459, t1461, t4158, t4162, t4165, t572, t573, param_d);
    (t13207, t13216, t13225, t13226, t13232, t13240, t13243, t13244, t13247, t13250)
}
