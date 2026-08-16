//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta605 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1943;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1944;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta605(t105944: f64, t1955: f64, t5978: f64, t886: f64, t1558: f64, t231: f64, t4533: f64, t6048: f64, t836: f64, t6071: f64, t105945: f64, t7063: f64, t18657: f64, t1579: f64, t4423: f64, t25207: f64, t77441: f64, t1544: f64, t580: f64, t98646: f64, t18435: f64, t27159: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t106275, t106290, t106302, t106360, t106365, t106387) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1943(t105944, t1955, t5978, t886, t1558, t231, t4533, t6048, t836, t6071, t105945, t7063);
        let (t106404, t106410, t106490, t106494, t106498) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1944(t18657, t1955, t1579, t231, t4423, t25207, t77441, t1544, t580, t98646, t18435, t27159);
    (t106275, t106290, t106302, t106360, t106365, t106387, t106404, t106410, t106490, t106494, t106498)
}
