//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3149/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3149(t19211: f64, t225: f64, t3507: f64, t6238: f64, t11914: f64, t1244: f64, t1246: f64, t14997: f64, t15022: f64, t15023: f64, t15027: f64, t15239: f64, t15245: f64, t15430: f64, t15771: f64, t15777: f64, t1734: f64, t1751: f64, t1755: f64, t19138: f64, t19166: f64, t19190: f64, t3493: f64, t3604: f64, t3624: f64, t3625: f64, t45326: f64, t475: f64, t5064: f64, t5072: f64, t53592: f64, t6252: f64, t6260: f64, t6739: f64) -> (f64, f64, f64) {
    let t65208 = t19211 * t225;
    let t65221 = t6238 * t3507;
    let t65249 = t11914 * t6252 * t6739 * t3493 * t475 + 2.0_f64 * t1244 * t1751 * t15239 * t1246 - t3624 * t65221 * t3625 - 2.0_f64 * t3624 * t1755 * t3625 * t15239 - 2.0_f64 * t15245 * t15023 + 8.0_f64 * t15027 * t14997 - t3624 * t6260 * t15022 + 2.0_f64 * t1244 * t15771 * t1734 * t1246 + 2.0_f64 * t3604 * t19190 + 4.0_f64 * t5064 * t15777 + 2.0_f64 * t53592 * t15430 - 4.0_f64 * t3624 * t5072 * t19138 + 12.0_f64 * t45326 * t19166;
    (t65208, t65221, t65249)
}
