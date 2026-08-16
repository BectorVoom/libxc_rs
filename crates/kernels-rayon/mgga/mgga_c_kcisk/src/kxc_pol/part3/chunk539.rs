//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 539/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk539(t524: f64, t4495: f64, t1589: f64, t1586: f64, t1567: f64, t1571: f64, t1568: f64, t1576: f64, t20: f64, t3914: f64, t533: f64, t1572: f64, t1580: f64, t1583: f64, t1593: f64, t4370: f64, t4378: f64, t4381: f64, t4385: f64, t4388: f64, t4393: f64, t4397: f64, t4403: f64, t4408: f64, t4411: f64, t4418: f64, t4421: f64, t535: f64, t541: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t536 = 0.0_f64 < t524;
    let t4497 = piecewise3(t536, t4495, -t4495);
    let t4498 = t1589 * t4497;
    let t4499 = t1586 * t4498;
    let t4502 = t1567 * t1571;
    let t4505 = t1568 * t1576;
    let t4509 = t3914 * t20;
    let t4510 = t533 * t4509;
    let t4513 = 0.2698618307426597582e-1_f64 * t4370 * t541 + 0.5397236614853195164e-1_f64 * t535 * t4378 - 0.47975436576472845902e-1_f64 * t4381 * t1583 + 0.59969295720591057378e-2_f64 * t4385 + 0.89953943580886586067e-2_f64 * t1580 * t4388 + 0.11993859144118211476e-1_f64 * t1580 * t4393 + 0.17990788716177317213e-1_f64 * t4397 * t1583 - 0.17990788716177317213e-1_f64 * t1580 * t4403 - 0.17990788716177317213e-1_f64 * t1580 * t4408 - 0.47975436576472845902e-1_f64 * t4411 + 0.14392630972941853771e0_f64 * t1572 * t1593 - t4418 - 0.17990788716177317213e-1_f64 * t4421 - 0.2698618307426597582e-1_f64 * t535 * t4499 - 0.14392630972941853771e0_f64 * t4502 * t541 + 0.17990788716177317213e-1_f64 * t4505 - 0.5397236614853195164e-1_f64 * t1568 * t1593 + 0.26386490117060065246e0_f64 * t4510 * t541;
    (t4497, t4498, t4499, t4502, t4509, t4510, t4513)
}
