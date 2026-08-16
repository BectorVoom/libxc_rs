//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1041/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1041(t13151: f64, t3210: f64, t3200: f64, t359: f64, t9494: f64, t13132: f64, t4554: f64, t10463: f64, t10466: f64, t13098: f64, t13102: f64, t13103: f64, t13108: f64, t13111: f64, t13115: f64, t13122: f64, t13126: f64, t13135: f64, t13139: f64, t13145: f64, t2836: f64, t3049: f64, t4782: f64, t9379: f64, t9383: f64, t9387: f64, t979: f64) -> (f64, f64, f64) {
    let t13152 = t3210 * t13151;
    let t13153 = t3200 * t13152;
    let t13155 = t359 * t9494;
    let t13156 = t13155 * t13132;
    let t13157 = t3210 * t13156;
    let t13158 = t4554 * t13157;
    let t13160 = -0.178244852896875e-2_f64 * t10463 * t13098 - t13102 + 0.22109259259259259258e-2_f64 * t13103 - 0.49745833333333333332e-2_f64 * t13108 - 0.24872916666666666666e-2_f64 * t13111 + 0.33163888888888888888e-2_f64 * t13115 + 0.22109259259259259258e-2_f64 * t9379 + 0.13345e0_f64 * t3049 * t4782 + 0.178089025e-1_f64 * t10466 * t4782 - 0.3684876543209876543e-3_f64 * t13122 - 0.24320185185185185185e-1_f64 * t13126 + 0.73697530864197530861e-2_f64 * t13135 - 0.22109259259259259258e-2_f64 * t13139 + 0.11054629629629629629e-2_f64 * t9383 + 0.18424382716049382715e-2_f64 * t9387 - 0.2671335375e-1_f64 * t2836 * t13098 + 0.66725e-1_f64 * t979 * t13145 - 0.13345e0_f64 * t979 * t13098 - 0.33163888888888888888e-2_f64 * t13153 - 0.16581944444444444444e-1_f64 * t13158;
    (t13153, t13158, t13160)
}
