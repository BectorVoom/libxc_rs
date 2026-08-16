//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta815 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2660;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2661;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta815(t20112: f64, t994: f64, t4746: f64, t4930: f64, t19855: f64, t993: f64, t378: f64, t15654: f64, t1678: f64, t225: f64, t11249: f64, t6299: f64, t1647: f64, t16565: f64, t12166: f64, t342: f64, t12077: f64, t20050: f64, t3106: f64, t1063: f64, t247: f64, t42447: f64, t6092: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t64737, t64764, t64817, t64845, t64907, t65144) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2660(t20112, t994, t4746, t4930, t19855, t993, t378, t15654, t1678, t225, t11249, t6299);
        let (t65181, t65216, t65220, t65288, t65292) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2661(t1647, t16565, t12166, t1678, t342, t12077, t20050, t3106, t1063, t247, t42447, t6092);
    (t64737, t64764, t64817, t64845, t64907, t65144, t65181, t65216, t65220, t65288, t65292)
}
