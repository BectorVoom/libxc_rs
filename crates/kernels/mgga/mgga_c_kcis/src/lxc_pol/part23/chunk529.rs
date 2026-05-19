//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 529/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk529<F: Float>(t4312: F, t4314: F, t3793: F, t3795: F, t3799: F, t3803: F, t3807: F, t1559: F, t1563: F, t1562: F, t597: F, t592: F) -> (F, F, F, F, F, F) {
    let t4315 = t4312 * t4314;
    let t4318 = F::cast_from(0.22831111111111111111e-1_f64) * t3793;
    let t4323 = t4318 + F::cast_from(0.11415555555555555555e-1_f64) * t3795 - F::cast_from(0.11415555555555555555e-1_f64) * t3799 + F::cast_from(0.34246666666666666666e-1_f64) * t3803 - F::cast_from(0.17123333333333333333e-1_f64) * t3807;
    let t4326 = t1559 * t1563;
    let t4329 = t1562 * t597;
    let t4330 = F::new(1.0) / t4329;
    let t4331 = t592 * t4330;
    (t4315, t4318, t4323, t4326, t4330, t4331)
}
