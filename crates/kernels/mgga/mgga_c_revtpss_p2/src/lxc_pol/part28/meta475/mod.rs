//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1799;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1800;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1801;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1802;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta475<F: Float>(t25411: F, t25413: F, t2718: F, t867: F, t1949: F, t2722: F, t2723: F, t1950: F, t2453: F, t2458: F, t231: F, t7076: F, t25372: F, t25410: F, t1959: F, t25362: F, t25364: F, t25366: F, t25368: F, t25371: F, t25379: F, t25383: F, t25388: F, t25391: F, t25395: F, t25400: F, t25406: F, t25407: F, t2829: F, t7053: F, t7070: F, t7073: F, t7079: F, t25360: F, t892: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25414, t25416, t25418, t25419, t25422, t25424, t25425, t25426) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1799::<F>(t25411, t25413, t2718, t867, t1949, t2722, t2723, t1950, t2453, t2458, t231, t7076);
        let t25431 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1800::<F>(t25372, t25410);
        let (t25432, t25434) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1801::<F>(t25413, t25431, t1959, t25362, t25364, t25366, t25368, t25371, t25379, t25383, t25388, t25391, t25395, t25400, t25406, t25407, t25414, t25419, t25424, t25426, t2829, t7053, t7070, t7073, t7079);
        let (t25435, t25436) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1802::<F>(t25360, t25434, t892);
    (t25414, t25416, t25418, t25419, t25422, t25424, t25425, t25426, t25431, t25432, t25435, t25436)
}
