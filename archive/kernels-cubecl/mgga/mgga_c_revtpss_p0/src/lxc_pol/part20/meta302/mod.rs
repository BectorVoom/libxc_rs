//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1187;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1188;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta302<F: Float>(t12393: F, t422: F, t12295: F, t12292: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F, t1151: F, t3427: F, t3384: F, t1149: F, t3435: F, t3433: F, t1160: F, t3444: F, t1156: F, t3476: F, t1170: F, t12233: F, t12240: F, t12242: F, t12245: F, t12251: F, t12360: F, t12363: F, t12366: F, t12379: F, t3447: F, t3472: F, t3480: F, t435: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t12395, t12408) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1187::<F>(t12393, t422, t12295, t12292, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320);
        let (t12411, t12413, t12415, t12417, t12418, t12423, t12426) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1188::<F>(t1151, t3427, t3384, t1149, t3435, t3433, t1160, t3444, t1156, t3476, t1170, t12233, t12240, t12242, t12245, t12251, t12360, t12363, t12366, t12379, t12395, t12408, t3447, t3472, t3480, t435);
    (t12395, t12408, t12411, t12413, t12415, t12417, t12418, t12423, t12426)
}
