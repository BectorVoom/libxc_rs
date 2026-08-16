//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1794;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1795;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1796;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1797;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta475(t25411: f64, t25413: f64, t2718: f64, t867: f64, t1949: f64, t2722: f64, t2723: f64, t1950: f64, t2453: f64, t2458: f64, t231: f64, t7076: f64, t25372: f64, t25410: f64, t1959: f64, t25362: f64, t25364: f64, t25366: f64, t25368: f64, t25371: f64, t25379: f64, t25383: f64, t25388: f64, t25391: f64, t25395: f64, t25400: f64, t25406: f64, t25407: f64, t2829: f64, t7053: f64, t7070: f64, t7073: f64, t7079: f64, t25360: f64, t892: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25414, t25416, t25418, t25419, t25422, t25424, t25425, t25426) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1794(t25411, t25413, t2718, t867, t1949, t2722, t2723, t1950, t2453, t2458, t231, t7076);
        let t25431 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1795(t25372, t25410);
        let (t25432, t25434) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1796(t25413, t25431, t1959, t25362, t25364, t25366, t25368, t25371, t25379, t25383, t25388, t25391, t25395, t25400, t25406, t25407, t25414, t25419, t25424, t25426, t2829, t7053, t7070, t7073, t7079);
        let (t25435, t25436) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1797(t25360, t25434, t892);
    (t25414, t25416, t25418, t25419, t25422, t25424, t25425, t25426, t25431, t25432, t25435, t25436)
}
