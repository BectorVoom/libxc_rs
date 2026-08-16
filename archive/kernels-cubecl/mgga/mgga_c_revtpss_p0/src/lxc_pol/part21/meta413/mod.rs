//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta413 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1885;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta413<F: Float>(t13225: F, t3: F, t2327: F, t670: F, t116: F, t2371: F, t10259: F, t117: F, t1459: F, t1461: F, t4158: F, t4162: F, t4165: F, t572: F, t573: F, param_d: F, t10270: F, t10272: F, t10279: F, t10281: F, t10288: F, t10290: F, t10275: F, t10278: F, t10284: F, t10287: F, t10295: F, t4171: F, t602: F) -> (F, F, F, F, F, F, F, F) {
        let (t13226, t13232, t13240, t13244, t13247, t13250) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1885::<F>(t13225, t3, t2327, t670, t116, t2371, t10259, t117, t1459, t1461, t4158, t4162, t4165, t572, t573, param_d);
        let (t13267, t13269) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1886::<F>(t10270, t10272, t10279, t10281, t10288, t10290, t10275, t10278, t10284, t10287, t10295, t4171, t602);
    (t13226, t13232, t13240, t13244, t13247, t13250, t13267, t13269)
}
