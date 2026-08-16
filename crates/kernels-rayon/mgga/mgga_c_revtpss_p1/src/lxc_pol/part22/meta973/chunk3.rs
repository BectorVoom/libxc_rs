//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3263/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3263(t14923: f64, t18521: f64, t10770: f64, t10943: f64, t18426: f64, t18469: f64, t18627: f64, t2646: f64, t2745: f64, t2747: f64, t4362: f64, t4364: f64, t50736: f64, t50740: f64, t50744: f64, t50748: f64, t50752: f64, t50754: f64) -> f64 {
    let t61952 = t14923 * t18521;
    let t61954 = 0.14291339372689912324e-4_f64 * t50736 - 0.11433071498151929859e-3_f64 * t50740 - 0.57165357490759649296e-4_f64 * t50744 - 0.25410001404642664112e-4_f64 * t50748 + 0.40015750243531754508e-1_f64 * t50752 - 0.80031500487063509016e-2_f64 * t50754 - 0.21437009059034868486e-3_f64 * t2745 * t4364 * t18426 * t2646 - 0.42874018118069736972e-2_f64 * t2745 * t10770 * t18469 * t2646 + 0.12862205435420921092e-2_f64 * t4362 * t4364 * t18426 * t10943 + 0.85748036236139473944e-3_f64 * t2745 * t2747 * t18627 * t2646 - 0.40015750243531754508e-2_f64 * t61952;
    t61954
}
