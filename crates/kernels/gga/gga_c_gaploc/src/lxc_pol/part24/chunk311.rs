//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 311/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk311<F: Float>(t1365: F, t1367: F, t448: F, t535: F, t105: F, t1063: F, t1126: F, t1138: F, t1161: F, t1307: F, t1312: F, t1325: F, t1331: F, t1341: F, t1345: F, t1349: F, t1354: F, t1358: F, t1361: F, t380: F, t419: F, t449: F, t478: F, t495: F) -> (F,) {
    let t1368 = t1365 * t1367;
    let t1371 = t535 * t448;
    let t1374 = -0.28455006635676149599e-1 * t105 * t1307 + 0.28455006635676149599e-1 * t105 * t1312 - 0.7588001769513639893e-1 * t380 * t449 + 0.7588001769513639893e-1 * t380 * t478 - t1126 + 0.56910013271352299198e-1 * t105 * t1325 + 0.28455006635676149599e-1 * t105 * t1331 - 0.7588001769513639893e-1 * t380 * t495 - 0.56910013271352299198e-1 * t419 * t495 + 0.56910013271352299198e-1 * t105 * t1341 - 0.85365019907028448797e-1 * t105 * t1345 + 0.63233348079280332442e-2 * t1349 * t1354 - 0.63233348079280332442e-2 * t1358 * t1361 + t1161 + 0.63233348079280332442e-2 * t1358 * t1368 - 0.56910013271352299198e-1 * t1063 * t1371 - t1138;
    (t1374,)
}
