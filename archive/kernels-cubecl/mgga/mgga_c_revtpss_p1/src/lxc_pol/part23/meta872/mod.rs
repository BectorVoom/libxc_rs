//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta872 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2772;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2773;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta872<F: Float>(t22026: F, t46929: F, t808: F, t22135: F, t9744: F, t1413: F, t21969: F, t547: F, t807: F, t221: F, t22274: F, t3978: F, t46716: F, t22279: F, t9921: F, t22255: F, t3930: F, t22259: F, t9976: F, t22125: F, t2713: F, t3964: F, t13848: F, t22096: F, t9816: F, t9818: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t74362, t74364, t74402, t74421) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2772::<F>(t22026, t46929, t808, t22135, t9744, t1413, t21969, t547, t807, t221, t22274, t3978, t46716);
        let (t74425, t74427, t74429, t74437, t74461) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2773::<F>(t221, t22279, t3978, t9921, t22255, t3930, t22259, t9976, t22125, t2713, t3964, t13848, t22096, t9816, t9818);
    (t74362, t74364, t74402, t74421, t74425, t74427, t74429, t74437, t74461)
}
