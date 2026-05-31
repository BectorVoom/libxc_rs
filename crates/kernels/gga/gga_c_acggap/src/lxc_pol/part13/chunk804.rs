//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 804/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk804<F: Float>(t1967: F, t2327: F, t7429: F, t7434: F, t7441: F, t7448: F, t7463: F, t8704: F, t8706: F, t8708: F, t8710: F, t8712: F, t8714: F, t8716: F, t8718: F, t8720: F) -> F {
    let t8722 = t1967 * t2327;
    let t8728 = F::cast_from(0.34299214494455789578e-2_f64) * t8704 - F::cast_from(0.85748036236139473944e-3_f64) * t8706 - F::cast_from(0.34299214494455789578e-2_f64) * t8708 + F::cast_from(0.17149607247227894789e-2_f64) * t8710 + F::cast_from(0.40015750243531754507e-2_f64) * t8712 - F::cast_from(0.40015750243531754507e-2_f64) * t8714 + F::cast_from(0.80031500487063509015e-2_f64) * t8716 - F::cast_from(0.17149607247227894789e-2_f64) * t8718 - F::cast_from(0.17149607247227894789e-2_f64) * t8720 - F::cast_from(0.64311027177104605458e-3_f64) * t8722 - F::cast_from(0.47172138434406228102e-3_f64) * t7429 - F::cast_from(0.94344276868812456204e-3_f64) * t7434 - F::cast_from(0.28015625e-1_f64) * t7441 - F::cast_from(0.420234375e-1_f64) * t7448 - t7463;
    t8728
}
