//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 928/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk928(t11765: f64, t9823: f64, t2536: f64, t3614: f64, t2009: f64, t2021: f64, t2684: f64, t45320: f64, t7585: f64, t10886: f64, t10984: f64, t11001: f64, t37166: f64, t43919: f64, t43922: f64, t44879: f64, t45337: f64, t45723: f64, t45725: f64, t45729: f64, t45731: f64, t45735: f64, t45736: f64, t45737: f64, t45741: f64, t45744: f64, t45747: f64, t45753: f64, t549: f64, t6066: f64, t7630: f64, t8793: f64) -> f64 {
    let t45755 = 0.35750489951850426669e0_f64 * t9823 * t11765;
    let t45758 = t2536 * t3614;
    let t45761 = 0.35750489951850426669e0_f64 * t2021 * t45758 * t2009;
    let t45766 = 0.87421871174939309262e2_f64 * t2684 * t7585 * t45320;
    let t45767 = -0.79445533226334281487e-1_f64 * t37166 * t549 * t44879 - 0.12780975317973583226e1_f64 * t45723 - 0.38342925953920749677e0_f64 * t45725 + 0.85206502119823888171e-1_f64 * t45729 - t45731 - t45735 + t45736 + t45737 - 0.76685851907841499353e0_f64 * t43919 - 0.76685851907841499353e0_f64 * t43922 + 0.51123901271894332903e0_f64 * t45741 + t45744 - t45747 - 0.14300195980740170668e1_f64 * t7630 * t6066 * t45337 - t45753 + t45755 + 0.21450293971110256002e1_f64 * t8793 * t10984 - t45761 + 0.71500979903700853338e0_f64 * t10886 * t11001 + t45766;
    t45767
}
