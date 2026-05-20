//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta163 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1038;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1039;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1040;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1041;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1042;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1043;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1044;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta163<F: Float>(t1312: F, t2320: F, t2322: F, t2327: F, t2371: F, t670: F, t93: F, t1330: F, t72: F, t757: F, t530: F, t566: F, t1353: F, t525: F, t605: F, t30: F, t2257: F, t513: F, t527: F, zeta_threshold: F, t1113: F, t33: F, t3351: F, t516: F, t162: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3821, t3825, t3826, t3827, t3828) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1038::<F>(t1312, t2320, t2322, t2327, t2371, t670, t93, t1330, t72, t757, t530, t566);
        let t3829 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1039::<F>(t1353);
        let t3833 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1040::<F>(t525);
        let t3834 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1041::<F>(t605);
        let (t3840, t3841) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1042::<F>(t30, t2257, t3833, t3834, t513, t527, zeta_threshold);
        let t3842 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1043::<F>(t1113);
        let t3850 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1044::<F>(t33, t3351, t3841, t3842, t516, t162, t3840, zeta_threshold);
    (t3821, t3825, t3826, t3827, t3828, t3829, t3833, t3834, t3841, t3842, t3850)
}
