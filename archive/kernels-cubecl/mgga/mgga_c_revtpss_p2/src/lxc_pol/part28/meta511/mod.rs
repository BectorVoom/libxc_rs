//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta511 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1911;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1912;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1913;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1914;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1915;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1916;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta511<F: Float>(t1580: F, t7014: F, t689: F, t27279: F, t7058: F, t72: F, t7769: F, t686: F, t25375: F, t25387: F, t1559: F, t886: F, t25392: F, t1955: F, t7057: F, t14495: F, t1949: F, t2718: F, t14587: F, t25383: F, t25388: F, t25391: F, t25400: F, t25406: F, t25414: F, t25424: F, t25432: F, t7083: F, t7766: F, t7770: F, t27272: F, t27297: F, t27329: F, t892: F, t2411: F, t7782: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t27334, t27335, t27338, t27340, t27341, t27342, t27344, t27349) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1911::<F>(t1580, t7014, t689, t27279, t7058, t72, t7769, t686, t25375, t25387, t1559, t886);
        let (t27350, t27353) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1912::<F>(t25392, t27349, t1955, t7057);
        let (t27354, t27357) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1913::<F>(t14495, t25392, t1949, t2718);
        let (t27358, t27361) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1914::<F>(t14587, t27357, t25383, t25388, t25391, t25400, t25406, t25414, t25424, t25432, t27335, t27338, t27342, t27344, t27350, t27353, t27354, t7083, t7766, t7770);
        let (t27363, t27364) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1915::<F>(t27272, t27297, t27329, t27361, t892);
        let t27368 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1916::<F>(t2411, t7782);
    (t27334, t27340, t27341, t27349, t27350, t27353, t27354, t27357, t27358, t27363, t27364, t27368)
}
