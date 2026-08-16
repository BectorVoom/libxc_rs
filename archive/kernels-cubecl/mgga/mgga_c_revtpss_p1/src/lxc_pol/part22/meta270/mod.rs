//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1655;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1656;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta270<F: Float>(t3: F, t6936: F, t116: F, t5883: F, t117: F, t5920: F, t1916: F, t1918: F, t572: F, t573: F, t640: F, t76: F, param_d: F, t159: F, t793: F, t1518: F, t94: F, t93: F, t587: F, t65: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t6937, t6941, t6945, t6948, t6951, t6977) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1655::<F>(t3, t6936, t116, t5883, t117, t5920, t1916, t1918, t572, t573, t640, t76, param_d);
        let (t7021, t7732, t7889, t8779) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1656::<F>(t159, t793, t1518, t94, t93, t587, t65);
    (t6937, t6941, t6945, t6948, t6951, t6977, t7021, t7732, t7889, t8779)
}
