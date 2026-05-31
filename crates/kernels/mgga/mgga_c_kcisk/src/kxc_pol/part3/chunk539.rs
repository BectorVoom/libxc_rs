//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 539/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk539<F: Float>(t524: F, t4495: F, t1589: F, t1586: F, t1567: F, t1571: F, t1568: F, t1576: F, t20: F, t3914: F, t533: F, t1572: F, t1580: F, t1583: F, t1593: F, t4370: F, t4378: F, t4381: F, t4385: F, t4388: F, t4393: F, t4397: F, t4403: F, t4408: F, t4411: F, t4418: F, t4421: F, t535: F, t541: F) -> (F, F, F, F, F, F, F) {
    let t536 = F::cast_from(0.0_f64) < t524;
    let t4497 = piecewise3::<F>(t536, t4495, -t4495);
    let t4498 = t1589 * t4497;
    let t4499 = t1586 * t4498;
    let t4502 = t1567 * t1571;
    let t4505 = t1568 * t1576;
    let t4509 = t3914 * t20;
    let t4510 = t533 * t4509;
    let t4513 = F::cast_from(0.2698618307426597582e-1_f64) * t4370 * t541 + F::cast_from(0.5397236614853195164e-1_f64) * t535 * t4378 - F::cast_from(0.47975436576472845902e-1_f64) * t4381 * t1583 + F::cast_from(0.59969295720591057378e-2_f64) * t4385 + F::cast_from(0.89953943580886586067e-2_f64) * t1580 * t4388 + F::cast_from(0.11993859144118211476e-1_f64) * t1580 * t4393 + F::cast_from(0.17990788716177317213e-1_f64) * t4397 * t1583 - F::cast_from(0.17990788716177317213e-1_f64) * t1580 * t4403 - F::cast_from(0.17990788716177317213e-1_f64) * t1580 * t4408 - F::cast_from(0.47975436576472845902e-1_f64) * t4411 + F::cast_from(0.14392630972941853771e0_f64) * t1572 * t1593 - t4418 - F::cast_from(0.17990788716177317213e-1_f64) * t4421 - F::cast_from(0.2698618307426597582e-1_f64) * t535 * t4499 - F::cast_from(0.14392630972941853771e0_f64) * t4502 * t541 + F::cast_from(0.17990788716177317213e-1_f64) * t4505 - F::cast_from(0.5397236614853195164e-1_f64) * t1568 * t1593 + F::cast_from(0.26386490117060065246e0_f64) * t4510 * t541;
    (t4497, t4498, t4499, t4502, t4509, t4510, t4513)
}
