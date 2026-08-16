//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta512 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1805;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1806;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1807;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1808;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta512<F: Float>(t30400: F, t7076: F, t2061: F, t231: F, t6016: F, t6048: F, t25317: F, t1956: F, t2067: F, t213: F, t257: F, t26534: F, t26536: F, t26538: F, t26557: F, t26578: F, t27199: F, t28422: F, t28434: F, t28449: F, t29698: F, t30357: F, t30381: F, t30384: F, t30392: F, t30396: F, t7070: F, t7766: F, t8007: F, t8016: F, t30355: F, t892: F, t1468: F, t1940: F, t2071: F, t2403: F, t26425: F, t26590: F, t28460: F, t29599: F, t29602: F, t29606: F, t29713: F, t29716: F, t29719: F, t30: F, t30317: F, t4541: F, t5824: F, t7432: F, t7749: F, t7787: F, t8020: F, t5966: F, t1544: F, t1583: F, t198: F, t207: F, t29598: F, t5962: F, t6075: F, t6079: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t30401, t30405, t30406, t30410, t30411, t30418) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1805::<F>(t30400, t7076, t2061, t231, t6016, t6048, t25317, t1956, t2067, t213, t257, t26534, t26536, t26538, t26557, t26578, t27199, t28422, t28434, t28449, t29698, t30357, t30381, t30384, t30392, t30396, t7070, t7766, t8007, t8016);
        let (t30419, t30420) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1806::<F>(t30355, t30418, t892);
        let t30438 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1807::<F>(t1468, t1940, t2071, t2403, t26425, t26590, t28460, t29599, t29602, t29606, t29713, t29716, t29719, t30, t30317, t30420, t4541, t5824, t7432, t7749, t7787, t8020);
        let (t30439, t30462) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1808::<F>(t2071, t5966, t1544, t1583, t1940, t198, t207, t2403, t26590, t28460, t29598, t30419, t4541, t5962, t6075, t6079, t7432, t8020, t892);
    (t30401, t30405, t30406, t30410, t30411, t30419, t30420, t30438, t30439, t30462)
}
