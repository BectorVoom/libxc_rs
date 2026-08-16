//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta265 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1188;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1189;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1190;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1191;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1192;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta265<F: Float>(t1096: F, t1976: F, t7160: F, t3140: F, t378: F, t1078: F, t1982: F, t1035: F, t1043: F, t1089: F, t1984: F, t359: F, t7135: F, t1000: F, t1097: F, t1978: F, t1983: F, t1986: F, t342: F, t7102: F, t7137: F, t7140: F, t7144: F, t7147: F, t7151: F, t7153: F, t7156: F, t7159: F, t989: F, t1989: F, t3336: F, t265: F, t393: F, t207: F, t7086: F, t1940: F, t1963: F, t198: F, t2403: F, t7091: F, t775: F, t890: F, t892: F, t1100: F, t1102: F, t336: F, t5023: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7161, t7162, t7166, t7167) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1188::<F>(t1096, t1976, t7160, t3140, t378, t1078, t1982);
        let t7168 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1189::<F>(t1035, t1976);
        let (t7170, t7174, t7177) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1190::<F>(t1043, t1089, t7168, t1984, t359, t7135, t1000, t1097, t1978, t1983, t1986, t342, t7102, t7137, t7140, t7144, t7147, t7151, t7153, t7156, t7159, t7162, t7167, t989);
        let t7181 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1191::<F>(t1989, t3336);
        let (t7188, t7193, t7194) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1192::<F>(t265, t393, t207, t7086, t1940, t1963, t198, t2403, t7091, t775, t890, t892, t1100, t1102, t336, t5023, t7177, t7181);
    (t7161, t7162, t7166, t7167, t7168, t7170, t7174, t7177, t7181, t7188, t7193, t7194)
}
