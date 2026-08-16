//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 331/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk331(t394: f64, t441: f64, t407: f64, t456: f64, t930: f64, t955: f64, t1210: f64, t182: f64, t119: f64, t1226: f64, t1228: f64, t1229: f64, t1231: f64, t1235: f64, t1238: f64, t1242: f64, t1246: f64, t1248: f64, t151: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1251 = t394 * t441;
    let t1252 = t1251 * t407;
    let t1255 = t456 * t930;
    let t1258 = t456 * t955;
    let t1261 = t182 * t1210;
    let t1264 = t1226 - t1228 - 0.13170898365871023197e1_f64 * t1229 + 0.13170898365871023197e1_f64 * t1231 + t1235 + 0.13170898365871023197e1_f64 * t1238 - 0.13170898365871023197e1_f64 * t1242 - t1246 + 0.13170898365871023197e1_f64 * t151 * t1248 - 0.13170898365871023197e1_f64 * t151 * t1252 - 0.65854491829355115987e0_f64 * t151 * t1255 - 0.65854491829355115987e0_f64 * t151 * t1258 + 0.65854491829355115987e0_f64 * t119 * t1261;
    (t1251, t1252, t1255, t1258, t1261, t1264)
}
