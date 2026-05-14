//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1134/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1134<F: Float>(t127: F, t368: F, t3751: F, t1477: F, t3754: F, t1319: F, t5654: F, t1482: F, t1419: F, t1650: F, t11634: F, t1444: F, t1102: F, t11632: F, t15991: F, t15994: F, t15997: F, t16001: F, t16003: F, t16006: F, t16010: F, t16014: F, t16018: F, t16022: F, t16026: F, t16031: F, t16035: F, t16038: F, t16349: F, t486: F) -> (F, F) {
    let t16353 = t127 * t368 * t3751;
    let t16354 = t1477 * t3754;
    let t16355 = t5654 * t1319;
    let t16356 = t16354 * t16355;
    let t16359 = t1482 * t3754;
    let t16360 = t5654 * t1419;
    let t16361 = t16359 * t16360;
    let t16364 = t1650 * t1319;
    let t16366 = t11634 * t16364 * t1419;
    let t16369 = t1477 * t1444;
    let t16370 = t16369 * t16355;
    let t16373 = t1482 * t1444;
    let t16374 = t16373 * t16360;
    let t16377 = 0.19711289e-2 * t1102 * t15991 + 0.21901432222222222221e-2 * t15994 - 0.7391733375e-3 * t1102 * t15997 + t16001 - t16003 + 0.1478346675e-2 * t1102 * t16006 + 0.7391733375e-3 * t1102 * t16010 - 0.19711289e-2 * t11632 * t16014 + 0.26281718666666666666e-2 * t11632 * t16018 + 0.98556445e-3 * t11632 * t16022 - 0.19711289e-2 * t11632 * t16026 - 0.295669335e-2 * t1102 * t16031 - 0.1478346675e-2 * t1102 * t16035 - 0.14600954814814814815e-3 * t16038 - 4.0 * t486 * t16349 + 0.32852148333333333333e-2 * t16353 * t16356 - 0.21901432222222222222e-2 * t16353 * t16361 - 0.19711289e-2 * t11632 * t16366 - 0.39422578e-2 * t11632 * t16370 + 0.26281718666666666666e-2 * t11632 * t16374;
    (t16360, t16377)
}
