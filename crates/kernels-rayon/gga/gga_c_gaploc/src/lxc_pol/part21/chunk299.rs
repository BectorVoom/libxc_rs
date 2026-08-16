//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 299/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk299(t1266: f64, t492: f64, t105: f64, t1063: f64, t1067: f64, t1076: f64, t1079: f64, t1083: f64, t1131: f64, t1153: f64, t1165: f64, t1169: f64, t1180: f64, t1188: f64, t1212: f64, t1217: f64, t1222: f64, t174: f64, t419: f64, t449: f64, t478: f64, t489: f64) -> f64 {
    let t1267 = t492 * t1266;
    let t1274 = 0.56910013271352299198e-1_f64 * t1063 * t1067 + t1076 + 0.12646669615856066488e-1_f64 * t1079 * t174 + 0.7588001769513639893e-1_f64 * t1083 * t174 + 0.28455006635676149599e-1_f64 * t1212 * t174 - t1180 - t1188 + t1169 + 0.31616674039640166222e-2_f64 * t1217 * t489 - 0.63233348079280332442e-2_f64 * t1222 * t489 - t1131 + t1165 + t1153 - 0.28455006635676149599e-1_f64 * t105 * t1267 - 0.56910013271352299198e-1_f64 * t419 * t449 + 0.56910013271352299198e-1_f64 * t419 * t478;
    t1274
}
