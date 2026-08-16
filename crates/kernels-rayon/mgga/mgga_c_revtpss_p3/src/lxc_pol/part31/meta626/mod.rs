//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta626 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2078;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2079;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta626(t1096: f64, t357: f64, t1976: f64, t4743: f64, t27543: f64, t342: f64, t4778: f64, t8521: f64, t1078: f64, t42859: f64, t1983: f64, t3143: f64, t1032: f64, t4930: f64, t994: f64, t15669: f64, t1035: f64, t25698: f64, t93920: f64, t1647: f64, t7135: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t99566, t99629, t99666, t99675, t99682, t99684) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2078(t1096, t357, t1976, t4743, t27543, t342, t4778, t8521, t1078, t42859, t1983, t3143);
        let (t99708, t99709, t99721, t99743, t99824, t99881) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2079(t1032, t4930, t994, t15669, t1976, t1035, t1983, t99682, t25698, t93920, t1647, t7135);
    (t99566, t99629, t99666, t99675, t99682, t99684, t99708, t99709, t99721, t99743, t99824, t99881)
}
