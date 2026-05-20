//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta335 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1257;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1258;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta335<F: Float>(t12211: F, t13206: F, t1310: F, t2371: F, t10192: F, t10194: F, t10260: F, t10263: F, t10415: F, t10416: F, t10426: F, t118: F, t1315: F, t1453: F, t2320: F, t2322: F, t2328: F, t2331: F, t2372: F, t3813: F, t3821: F, t4151: F, t4254: F, t508: F, t511: F, t569: F, t649: F, t651: F, t671: F, t3: F, t2327: F, t670: F, t116: F, t10259: F, t117: F, t1459: F, t1461: F, t4158: F, t4162: F, t4165: F, t572: F, t573: F, param_d: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13207, t13216, t13225) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1257::<F>(t12211, t13206, t1310, t2371, t10192, t10194, t10260, t10263, t10415, t10416, t10426, t118, t1315, t1453, t2320, t2322, t2328, t2331, t2372, t3813, t3821, t4151, t4254, t508, t511, t569, t649, t651, t671);
        let (t13226, t13232, t13240, t13243, t13244, t13247, t13250) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1258::<F>(t13225, t3, t2327, t670, t116, t2371, t10259, t117, t1459, t1461, t4158, t4162, t4165, t572, t573, param_d);
    (t13207, t13216, t13225, t13226, t13232, t13240, t13243, t13244, t13247, t13250)
}
