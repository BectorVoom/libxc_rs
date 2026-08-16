//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1716/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1716(t1119: f64, t18686: f64, t14845: f64, t1671: f64, t4740: f64, t4782: f64, t11424: f64, t5989: f64, t3259: f64, t6021: f64, t11136: f64, t11137: f64, t14702: f64, t14922: f64, t14923: f64, t14924: f64, t18203: f64, t18208: f64, t18213: f64, t18217: f64, t18219: f64, t18223: f64, t18227: f64, t18229: f64, t18234: f64, t18239: f64, t18243: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18688 = 1.0_f64 * t18686 * t1119;
    let t18690 = 2.0_f64 * t14845 * t1671;
    let t18692 = 2.0_f64 * t4740 * t4782;
    let t18694 = 2.0_f64 * t11424 * t5989;
    let t18696 = 1.0_f64 * t3259 * t6021;
    let t18710 = -t11136 + 0.41203703703703703703e-2_f64 * t11137 + 0.82407407407407407408e-2_f64 * t14702 + t14922 - t14923 - t14924 + 0.20601851851851851852e-2_f64 * t18203 + 0.10300925925925925926e-1_f64 * t18208 - 0.37083333333333333333e-1_f64 * t18213 - 0.12361111111111111111e-1_f64 * t18217 - 0.61805555555555555557e-2_f64 * t18219 + 0.55625000000000000001e-1_f64 * t18223 + 0.37083333333333333334e-1_f64 * t18227 - 0.30902777777777777778e-2_f64 * t18229 - 0.61805555555555555555e-2_f64 * t18234 + 0.18541666666666666667e-1_f64 * t18239 + 0.92708333333333333333e-2_f64 * t18243;
    (t18688, t18690, t18692, t18694, t18696, t18710)
}
