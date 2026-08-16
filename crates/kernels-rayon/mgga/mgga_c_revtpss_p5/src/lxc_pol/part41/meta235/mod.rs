//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta235 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk904;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk905;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk906;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk907;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk908;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta235(t378: f64, t6305: f64, t3304: f64, t1089: f64, t1668: f64, t1678: f64, t6299: f64, t3318: f64, t380: f64, t6343: f64, t1024: f64, t1087: f64, t1647: f64, t1685: f64, t1689: f64, t1692: f64, t3204: f64, t3287: f64, t3299: f64, t3317: f64, t342: f64, t381: f64, t4857: f64, t4954: f64, t6235: f64, t6362: f64, t6365: f64, t6368: f64, t6371: f64, t1079: f64, t1076: f64, t1652: f64, t1680: f64, t1696: f64, t3058: f64, t386: f64, t4747: f64, t4752: f64, t4778: f64, t4935: f64, t6245: f64, t6251: f64, t6259: f64, t6345: f64, t6351: f64, t995: f64, t1699: f64, t1102: f64, t198: f64, t3336: f64, t336: f64, t6106: f64, t6108: f64, t6112: f64, t6144: f64, t6147: f64, t6213: f64, t6215: f64, t6217: f64, t6221: f64, t6225: f64, t6229: f64, t30: f64, t265: f64, t393: f64, t6084: f64, t1468: f64, t1469: f64, t1587: f64, t1704: f64, t395: f64, t45: f64, t5824: f64, t5825: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6375, t6379, t6383, t6386, t6389, t6392) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk904(t378, t6305, t3304, t1089, t1668, t1678, t6299, t3318, t380, t6343, t1024, t1087, t1647, t1685, t1689, t1692, t3204, t3287, t3299, t3317, t342, t381, t4857, t4954, t6235, t6362, t6365, t6368, t6371);
        let (t6393, t6396) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk905(t1079, t6392, t1076, t1647, t1652, t1680, t1696, t3058, t342, t386, t4747, t4752, t4778, t4935, t6235, t6245, t6251, t6259, t6345, t6351, t995);
        let (t6400, t6404) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk906(t1699, t1102, t198, t3336, t336, t6106, t6108, t6112, t6144, t6147, t6213, t6215, t6217, t6221, t6225, t6229, t6396);
        let (t6405, t6412) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk907(t30, t265, t393, t6084, t6404, t1468, t1469, t1587, t1704, t395, t45, t5824, t5825, dens_threshold, rho0, zeta_threshold);
        let t6416 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk908(t5824);
    (t6375, t6379, t6383, t6386, t6389, t6392, t6393, t6396, t6400, t6405, t6412, t6416)
}
