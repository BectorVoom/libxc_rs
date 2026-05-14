//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1104/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1104<F: Float>(t322: F, t40851: F, t1083: F, t1085: F, t1087: F, t1089: F, t2412: F, t3390: F, t3394: F, t3398: F, t3402: F, t8440: F, t8463: F, t8465: F, t1020: F, t1091: F, t11092: F, t11118: F, t11120: F, t1310: F, t1312: F, t2410: F, t3406: F, t3652: F, t3656: F, t3660: F, t3664: F, t3668: F, t8438: F) -> (F, F, F) {
    let t332 = 0.25e1 < t322;
    let t40893 = piecewise3(t332, 0.0, t40851);
    let t40923 = -0.64e0 * t40893 + 0.1469548921044e3 * t3390 * t2412 + 0.1469548921044e3 * t1083 * t8465 + 0.734774460522e2 * t1083 * t8463 - 0.22988522834472e3 * t3394 * t2412 - 0.22988522834472e3 * t1085 * t8465 - 0.11494261417236e3 * t1085 * t8463 + 0.12405227240928e3 * t3398 * t2412 + 0.12405227240928e3 * t1087 * t8465 + 0.6202613620464e2 * t1087 * t8463 - 0.2177652951264e2 * t3402 * t2412 - 0.2177652951264e2 * t1089 * t8465 - 0.1088826475632e2 * t1089 * t8463 - 0.22988522834472e3 * t1083 * t8440 + 0.18607840861392e3 * t1085 * t8440;
    let t40954 = -0.4355305902528e2 * t1087 * t8440 + 0.6202613620464e2 * t3660 * t1312 - 0.1088826475632e2 * t3664 * t1312 + 0.734774460522e2 * t11092 * t1020 + 0.734774460522e2 * t3652 * t1312 - 0.11494261417236e3 * t3656 * t1312 - 0.3831420472412e2 * t3660 * t1310 + 0.1550653405116e2 * t11118 * t1020 + 0.3101306810232e2 * t3402 * t2410 + 0.1550653405116e2 * t1089 * t8438 + 0.1550653405116e2 * t3664 * t1310 - 0.2177652951264e1 * t11120 * t1020 - 0.4355305902528e1 * t3406 * t2410 - 0.2177652951264e1 * t1091 * t8438 - 0.2177652951264e1 * t3668 * t1310;
    (t40893, t40923, t40954)
}
