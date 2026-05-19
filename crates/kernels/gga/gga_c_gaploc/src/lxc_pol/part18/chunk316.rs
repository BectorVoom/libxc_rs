//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 316/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk316<F: Float>(t1365: F, t1367: F, t448: F, t535: F, t105: F, t1063: F, t1126: F, t1138: F, t1161: F, t1307: F, t1312: F, t1325: F, t1331: F, t1341: F, t1345: F, t1349: F, t1354: F, t1358: F, t1361: F, t380: F, t419: F, t449: F, t478: F, t495: F) -> F {
    let t1368 = t1365 * t1367;
    let t1371 = t535 * t448;
    let t1374 = -F::cast_from(0.28455006635676149599e-1_f64) * t105 * t1307 + F::cast_from(0.28455006635676149599e-1_f64) * t105 * t1312 - F::cast_from(0.7588001769513639893e-1_f64) * t380 * t449 + F::cast_from(0.7588001769513639893e-1_f64) * t380 * t478 - t1126 + F::cast_from(0.56910013271352299198e-1_f64) * t105 * t1325 + F::cast_from(0.28455006635676149599e-1_f64) * t105 * t1331 - F::cast_from(0.7588001769513639893e-1_f64) * t380 * t495 - F::cast_from(0.56910013271352299198e-1_f64) * t419 * t495 + F::cast_from(0.56910013271352299198e-1_f64) * t105 * t1341 - F::cast_from(0.85365019907028448797e-1_f64) * t105 * t1345 + F::cast_from(0.63233348079280332442e-2_f64) * t1349 * t1354 - F::cast_from(0.63233348079280332442e-2_f64) * t1358 * t1361 + t1161 + F::cast_from(0.63233348079280332442e-2_f64) * t1358 * t1368 - F::cast_from(0.56910013271352299198e-1_f64) * t1063 * t1371 - t1138;
    t1374
}
