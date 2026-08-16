//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta509 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1839;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1840;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1841;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1842;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1843;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1844;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1845;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta509<F: Float>(t14587: F, t27357: F, t25383: F, t25388: F, t25391: F, t25400: F, t25406: F, t25414: F, t25424: F, t25432: F, t27335: F, t27338: F, t27342: F, t27344: F, t27350: F, t27353: F, t27354: F, t7083: F, t7766: F, t7770: F, t27272: F, t27297: F, t27329: F, t892: F, t2411: F, t7782: F, t1583: F, t775: F, t25207: F, t198: F, t1993: F, t11064: F, t30: F, t890: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t27358, t27361) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1839::<F>(t14587, t27357, t25383, t25388, t25391, t25400, t25406, t25414, t25424, t25432, t27335, t27338, t27342, t27344, t27350, t27353, t27354, t7083, t7766, t7770);
        let (t27363, t27364) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1840::<F>(t27272, t27297, t27329, t27361, t892);
        let t27368 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1841::<F>(t2411, t7782);
        let t27375 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1842::<F>(t1583, t775);
        let (t27376, t27382) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1843::<F>(t25207, t27375, t198, t1993);
        let t27383 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1844::<F>(t11064, t30);
        let t27384 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1845::<F>(t1583, t890);
    (t27358, t27363, t27364, t27368, t27375, t27376, t27382, t27383, t27384)
}
