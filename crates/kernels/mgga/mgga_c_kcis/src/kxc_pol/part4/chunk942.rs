//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 942/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk942<F: Float>(t359: F, t9372: F, t1646: F, t2630: F, t3210: F, t13130: F, t1804: F, t2850: F, t3200: F, t3045: F, t4781: F, t2635: F, t4546: F, t9494: F, t4554: F, t10463: F, t10466: F, t13098: F, t13102: F, t13103: F, t13108: F, t13111: F, t13115: F, t13122: F, t13126: F, t2836: F, t3049: F, t4782: F, t9379: F, t9383: F, t9387: F, t979: F) -> (F, F, F, F, F, F, F, F) {
    let t13131 = t359 * t9372;
    let t13132 = t1646 * t2630;
    let t13133 = t13131 * t13132;
    let t13134 = t3210 * t13133;
    let t13135 = t13130 * t13134;
    let t13137 = t1804 * t2850;
    let t13138 = t3210 * t13137;
    let t13139 = t3200 * t13138;
    let t13145 = t4781 * t3045;
    let t13150 = t1646 * t2635;
    let t13151 = t4546 * t13150;
    let t13152 = t3210 * t13151;
    let t13153 = t3200 * t13152;
    let t13155 = t359 * t9494;
    let t13156 = t13155 * t13132;
    let t13157 = t3210 * t13156;
    let t13158 = t4554 * t13157;
    let t13160 = -0.178244852896875e-2 * t10463 * t13098 - t13102 + 0.22109259259259259258e-2 * t13103 - 0.49745833333333333332e-2 * t13108 - 0.24872916666666666666e-2 * t13111 + 0.33163888888888888888e-2 * t13115 + 0.22109259259259259258e-2 * t9379 + 0.13345e0 * t3049 * t4782 + 0.178089025e-1 * t10466 * t4782 - 0.3684876543209876543e-3 * t13122 - 0.24320185185185185185e-1 * t13126 + 0.73697530864197530861e-2 * t13135 - 0.22109259259259259258e-2 * t13139 + 0.11054629629629629629e-2 * t9383 + 0.18424382716049382715e-2 * t9387 - 0.2671335375e-1 * t2836 * t13098 + 0.66725e-1 * t979 * t13145 - 0.13345e0 * t979 * t13098 - 0.33163888888888888888e-2 * t13153 - 0.16581944444444444444e-1 * t13158;
    (t13132, t13135, t13139, t13145, t13150, t13153, t13158, t13160)
}
