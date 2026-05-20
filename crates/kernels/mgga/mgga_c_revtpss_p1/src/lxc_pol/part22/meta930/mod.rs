//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta930 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3157;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3158;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta930<F: Float>(t12772: F, t17673: F, t3625: F, t12910: F, t12916: F, t17460: F, t17213: F, t3172: F, t5384: F, t13069: F, t5265: F, t1260: F, t17332: F, t17747: F, t17749: F, t1222: F, t16725: F, t17471: F, t16729: F, t13017: F, t5373: F, t44546: F, t5331: F, t5334: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t57170, t57173, t57176, t57178, t57187) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3157::<F>(t12772, t17673, t3625, t12910, t12916, t17460, t17213, t3172, t5384, t13069, t5265, t1260, t17332);
        let (t57191, t57209, t57212, t57214, t57222) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3158::<F>(t12916, t17747, t17749, t1222, t16725, t17471, t16729, t13017, t5373, t44546, t5331, t5334);
    (t57170, t57173, t57176, t57178, t57187, t57191, t57209, t57212, t57214, t57222)
}
