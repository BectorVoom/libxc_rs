//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta288 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1516;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1517;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1518;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1519;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta288<F: Float>(t1071: F, t3057: F, t992: F, t338: F, t378: F, t3056: F, t988: F, t1031: F, t342: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11187, t11198, t11199, t11200) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1516::<F>(t1071, t3057, t992, t338);
        let (t11201, t11223) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1517::<F>(t11200, t378, t3056, t988);
        let (t11224, t11238, t11239) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1518::<F>(t11223, t378, t1031);
        let t11240 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1519::<F>(t11239, t342);
    (t11187, t11198, t11199, t11200, t11201, t11223, t11224, t11238, t11239, t11240)
}
