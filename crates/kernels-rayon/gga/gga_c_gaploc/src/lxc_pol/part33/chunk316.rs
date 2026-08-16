//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 316/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk316(t1365: f64, t1367: f64, t448: f64, t535: f64, t105: f64, t1063: f64, t1126: f64, t1138: f64, t1161: f64, t1307: f64, t1312: f64, t1325: f64, t1331: f64, t1341: f64, t1345: f64, t1349: f64, t1354: f64, t1358: f64, t1361: f64, t380: f64, t419: f64, t449: f64, t478: f64, t495: f64) -> f64 {
    let t1368 = t1365 * t1367;
    let t1371 = t535 * t448;
    let t1374 = -0.28455006635676149599e-1_f64 * t105 * t1307 + 0.28455006635676149599e-1_f64 * t105 * t1312 - 0.7588001769513639893e-1_f64 * t380 * t449 + 0.7588001769513639893e-1_f64 * t380 * t478 - t1126 + 0.56910013271352299198e-1_f64 * t105 * t1325 + 0.28455006635676149599e-1_f64 * t105 * t1331 - 0.7588001769513639893e-1_f64 * t380 * t495 - 0.56910013271352299198e-1_f64 * t419 * t495 + 0.56910013271352299198e-1_f64 * t105 * t1341 - 0.85365019907028448797e-1_f64 * t105 * t1345 + 0.63233348079280332442e-2_f64 * t1349 * t1354 - 0.63233348079280332442e-2_f64 * t1358 * t1361 + t1161 + 0.63233348079280332442e-2_f64 * t1358 * t1368 - 0.56910013271352299198e-1_f64 * t1063 * t1371 - t1138;
    t1374
}
