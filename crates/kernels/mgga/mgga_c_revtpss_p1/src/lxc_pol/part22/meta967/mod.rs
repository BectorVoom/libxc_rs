//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta967 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3231;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3232;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta967<F: Float>(t14622: F, t18259: F, t18281: F, t189: F, t4401: F, t606: F, t190: F, t2611: F, t60717: F, t18555: F, t2619: F, t13396: F, t14330: F, t4402: F, t50113: F, t40150: F, t14341: F, t4311: F, t18253: F, t18268: F, t198: F, t2394: F, t2430: F, t262: F, t39989: F, t4541: F, t50080: F, t5966: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t61265, t61269, t61274, t61283, t61286) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3231::<F>(t14622, t18259, t18281, t189, t4401, t606, t190, t2611, t60717, t18555, t2619, t13396, t14330, t4402);
        let (t61287, t61288, t61290, t61291) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3232::<F>(t50113, t40150, t14341, t4311, t18253, t18268, t198, t2394, t2430, t262, t39989, t4541, t50080, t5966, t61265, t61269, t61274, t61283, t61286);
    (t61265, t61269, t61274, t61283, t61286, t61287, t61288, t61290, t61291)
}
