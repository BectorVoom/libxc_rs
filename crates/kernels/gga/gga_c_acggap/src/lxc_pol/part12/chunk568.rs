//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 568/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk568<F: Float>(t1131: F, t540: F, t960: F, t1313: F, t839: F, t922: F, t1137: F, t1324: F, t1140: F, t1328: F, t1322: F, t1350: F, t398: F, t429: F, t384: F, t1150: F, t3215: F, t3218: F, t3229: F, t3231: F, t3233: F, t3235: F, t3238: F, t3240: F, t3246: F, t3271: F, t3273: F, t3280: F, t3293: F, t335: F, t3616: F, t367: F) -> (F, F, F, F, F, F) {
    let t4479 = t540 * t1131;
    let t4480 = t960 * t4479;
    let t4483 = t1313 * t839;
    let t4484 = t960 * t4483;
    let t4487 = t1313 * t922;
    let t4488 = t960 * t4487;
    let t4492 = 7.0 / 72.0 * t1137 * t1324;
    let t4494 = 7.0 / 72.0 * t1140 * t1328;
    let t4495 = t1322 * t839;
    let t4496 = t960 * t4495;
    let t4503 = t398 * t429 * t1350;
    let t4505 = 0.85748036236139473944e-3 * t384 * t4503;
    let t4507 = -t3215 - t3218 - 0.17149607247227894789e-2 * t3229 + 0.85748036236139473944e-3 * t3231 - 0.85748036236139473944e-3 * t3233 + 0.40015750243531754508e-2 * t3235 - 0.80031500487063509016e-2 * t3238 + 0.80031500487063509016e-2 * t3240 - t3246 + t367 * t4480 / 48.0 + t1150 * t4484 / 16.0 - t3616 * t4488 / 4.0 - t4492 - t4494 + t335 * t4496 / 48.0 + 0.42874018118069736972e-3 * t3271 - 0.85748036236139473944e-3 * t3273 - 0.20007875121765877254e-2 * t3280 - t4505 - 0.12862205435420921092e-2 * t3293;
    (t4479, t4483, t4487, t4495, t4503, t4507)
}
