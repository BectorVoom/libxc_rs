//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1187;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1188;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta302(t12393: f64, t422: f64, t12295: f64, t12292: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t12314: f64, t12317: f64, t12320: f64, t1151: f64, t3427: f64, t3384: f64, t1149: f64, t3435: f64, t3433: f64, t1160: f64, t3444: f64, t1156: f64, t3476: f64, t1170: f64, t12233: f64, t12240: f64, t12242: f64, t12245: f64, t12251: f64, t12360: f64, t12363: f64, t12366: f64, t12379: f64, t3447: f64, t3472: f64, t3480: f64, t435: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12395, t12408) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1187(t12393, t422, t12295, t12292, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320);
        let (t12411, t12413, t12415, t12417, t12418, t12423, t12426) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1188(t1151, t3427, t3384, t1149, t3435, t3433, t1160, t3444, t1156, t3476, t1170, t12233, t12240, t12242, t12245, t12251, t12360, t12363, t12366, t12379, t12395, t12408, t3447, t3472, t3480, t435);
    (t12395, t12408, t12411, t12413, t12415, t12417, t12418, t12423, t12426)
}
