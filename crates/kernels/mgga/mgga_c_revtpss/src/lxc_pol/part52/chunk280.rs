//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 280/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk280<F: Float>(t30: F, t33: F, t1330: F, t189: F, t512: F, t520: F, t749: F, t187: F, t72: F, t757: F, t177: F, t762: F, t531: F, t566: F, t513: F, t605: F, t516: F, t1113: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t1331 = t1330 * t189;
    let t1332 = t512 * t1331;
    let t1333 = t520 * t749;
    let t1334 = t512 * t1333;
    let t1336 = 0.19751673498613801407e-1 * t1330 * t187;
    let t1337 = t520 * t72;
    let t1339 = 0.18311447306006545054e-3 * t1337 * t757;
    let t1340 = t520 * t177;
    let t1342 = 0.5848223622634646207e0 * t1340 * t762;
    let t1343 = t531 * t566;
    let t1344 = 1.0 / t513;
    let t1347 = piecewise3(t31, 0.0, 2.0 / 3.0 * t1344 * t605);
    let t1348 = 1.0 / t516;
    let t1351 = piecewise3(t34, 0.0, 2.0 / 3.0 * t1348 * t1113);
    (t1331, t1332, t1333, t1334, t1336, t1337, t1339, t1340, t1342, t1343, t1344, t1347, t1348, t1351)
}
