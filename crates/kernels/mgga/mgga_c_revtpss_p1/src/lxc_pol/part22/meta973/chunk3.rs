//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3263/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3263<F: Float>(t14923: F, t18521: F, t10770: F, t10943: F, t18426: F, t18469: F, t18627: F, t2646: F, t2745: F, t2747: F, t4362: F, t4364: F, t50736: F, t50740: F, t50744: F, t50748: F, t50752: F, t50754: F) -> F {
    let t61952 = t14923 * t18521;
    let t61954 = F::cast_from(0.14291339372689912324e-4_f64) * t50736 - F::cast_from(0.11433071498151929859e-3_f64) * t50740 - F::cast_from(0.57165357490759649296e-4_f64) * t50744 - F::cast_from(0.25410001404642664112e-4_f64) * t50748 + F::cast_from(0.40015750243531754508e-1_f64) * t50752 - F::cast_from(0.80031500487063509016e-2_f64) * t50754 - F::cast_from(0.21437009059034868486e-3_f64) * t2745 * t4364 * t18426 * t2646 - F::cast_from(0.42874018118069736972e-2_f64) * t2745 * t10770 * t18469 * t2646 + F::cast_from(0.12862205435420921092e-2_f64) * t4362 * t4364 * t18426 * t10943 + F::cast_from(0.85748036236139473944e-3_f64) * t2745 * t2747 * t18627 * t2646 - F::cast_from(0.40015750243531754508e-2_f64) * t61952;
    t61954
}
