//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta512 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1805;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1806;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1807;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1808;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta512(t30400: f64, t7076: f64, t2061: f64, t231: f64, t6016: f64, t6048: f64, t25317: f64, t1956: f64, t2067: f64, t213: f64, t257: f64, t26534: f64, t26536: f64, t26538: f64, t26557: f64, t26578: f64, t27199: f64, t28422: f64, t28434: f64, t28449: f64, t29698: f64, t30357: f64, t30381: f64, t30384: f64, t30392: f64, t30396: f64, t7070: f64, t7766: f64, t8007: f64, t8016: f64, t30355: f64, t892: f64, t1468: f64, t1940: f64, t2071: f64, t2403: f64, t26425: f64, t26590: f64, t28460: f64, t29599: f64, t29602: f64, t29606: f64, t29713: f64, t29716: f64, t29719: f64, t30: f64, t30317: f64, t4541: f64, t5824: f64, t7432: f64, t7749: f64, t7787: f64, t8020: f64, t5966: f64, t1544: f64, t1583: f64, t198: f64, t207: f64, t29598: f64, t5962: f64, t6075: f64, t6079: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30401, t30405, t30406, t30410, t30411, t30418) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1805(t30400, t7076, t2061, t231, t6016, t6048, t25317, t1956, t2067, t213, t257, t26534, t26536, t26538, t26557, t26578, t27199, t28422, t28434, t28449, t29698, t30357, t30381, t30384, t30392, t30396, t7070, t7766, t8007, t8016);
        let (t30419, t30420) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1806(t30355, t30418, t892);
        let t30438 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1807(t1468, t1940, t2071, t2403, t26425, t26590, t28460, t29599, t29602, t29606, t29713, t29716, t29719, t30, t30317, t30420, t4541, t5824, t7432, t7749, t7787, t8020);
        let (t30439, t30462) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1808(t2071, t5966, t1544, t1583, t1940, t198, t207, t2403, t26590, t28460, t29598, t30419, t4541, t5962, t6075, t6079, t7432, t8020, t892);
    (t30401, t30405, t30406, t30410, t30411, t30419, t30420, t30438, t30439, t30462)
}
