//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta482 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1939;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1940;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1941;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta482(t30: f64, t265: f64, t393: f64, t18884: f64, t19141: f64, t20234: f64, t1106: f64, t1468: f64, t1469: f64, t1704: f64, t18280: f64, t18281: f64, t18892: f64, t395: f64, t4186: f64, t45: f64, t4560: f64, t5028: f64, t5824: f64, t5825: f64, t605: f64, t606: f64, t6084: f64, t6405: f64, t895: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t3531: f64, t6556: f64, t6552: f64, t3362: f64, t3417: f64, t141: f64, t1121: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20236, t20248) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1939(t30, t265, t393, t18884, t19141, t20234, t1106, t1468, t1469, t1704, t18280, t18281, t18892, t395, t4186, t45, t4560, t5028, t5824, t5825, t605, t606, t6084, t6405, t895, dens_threshold, rho0, zeta_threshold);
        let t20256 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1940(t18280);
        let (t20261, t20263, t20265, t20266, t20267, t20268, t20272) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1941(t3531, t6556, t6552, t3362, t5825, t606, t3417, t141, t1121, t18281);
    (t20236, t20248, t20256, t20261, t20263, t20265, t20266, t20267, t20268, t20272)
}
