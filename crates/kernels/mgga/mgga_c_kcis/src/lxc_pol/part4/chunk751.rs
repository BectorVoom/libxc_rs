//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 751/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk751<F: Float>(t169: F, t449: F, t4504: F, t446: F, t1646: F, t2629: F, t167: F, t171: F, t740: F, t829: F, t1650: F, t2641: F, t176: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t4505 = t449 * t4504;
    let t4506 = t446 * t4505;
    let t4507 = t4506 / F::cast_from(16.0_f64);
    let t4510 = t2629 * t1646;
    let t4513 = t171 * t167;
    let t4517 = piecewise3::<F>(t170, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t4510 * t829 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t4513 * t740);
    let t4518 = t2641 * t1650;
    let t4521 = t176 * t167;
    (t4505, t4507, t4510, t4513, t4517, t4518, t4521)
}
