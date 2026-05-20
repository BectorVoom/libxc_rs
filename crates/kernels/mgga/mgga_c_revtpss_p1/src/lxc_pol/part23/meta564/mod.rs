//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta564 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2136;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2137;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta564<F: Float>(t117: F, t22746: F, t1312: F, t1518: F, t18245: F, t22633: F, t22639: F, t4248: F, t5920: F, t7889: F, t13584: F, t22186: F, t22188: F, t22191: F, t22196: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9374: F, t9389: F, t9391: F) -> (F, F, F, F, F, F, F, F) {
        let (t22747, t22758, t22762, t22763) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2136::<F>(t117, t22746, t1312, t1518, t18245, t22633, t22639, t4248, t5920, t7889, t13584, t22186);
        let (t22764, t22765, t22766, t22767) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2137::<F>(t22188, t22191, t22196, t22762, t22763, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391);
    (t22747, t22758, t22762, t22763, t22764, t22765, t22766, t22767)
}
