//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta265 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1188;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1189;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1190;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1191;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1192;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta265(t1096: f64, t1976: f64, t7160: f64, t3140: f64, t378: f64, t1078: f64, t1982: f64, t1035: f64, t1043: f64, t1089: f64, t1984: f64, t359: f64, t7135: f64, t1000: f64, t1097: f64, t1978: f64, t1983: f64, t1986: f64, t342: f64, t7102: f64, t7137: f64, t7140: f64, t7144: f64, t7147: f64, t7151: f64, t7153: f64, t7156: f64, t7159: f64, t989: f64, t1989: f64, t3336: f64, t265: f64, t393: f64, t207: f64, t7086: f64, t1940: f64, t1963: f64, t198: f64, t2403: f64, t7091: f64, t775: f64, t890: f64, t892: f64, t1100: f64, t1102: f64, t336: f64, t5023: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7161, t7162, t7166, t7167) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1188(t1096, t1976, t7160, t3140, t378, t1078, t1982);
        let t7168 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1189(t1035, t1976);
        let (t7170, t7174, t7177) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1190(t1043, t1089, t7168, t1984, t359, t7135, t1000, t1097, t1978, t1983, t1986, t342, t7102, t7137, t7140, t7144, t7147, t7151, t7153, t7156, t7159, t7162, t7167, t989);
        let t7181 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1191(t1989, t3336);
        let (t7188, t7193, t7194) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1192(t265, t393, t207, t7086, t1940, t1963, t198, t2403, t7091, t775, t890, t892, t1100, t1102, t336, t5023, t7177, t7181);
    (t7161, t7162, t7166, t7167, t7168, t7170, t7174, t7177, t7181, t7188, t7193, t7194)
}
