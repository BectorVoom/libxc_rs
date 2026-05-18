//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 928/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk928<F: Float>(t11765: F, t9823: F, t2536: F, t3614: F, t2009: F, t2021: F, t2684: F, t45320: F, t7585: F, t10886: F, t10984: F, t11001: F, t37166: F, t43919: F, t43922: F, t44879: F, t45337: F, t45723: F, t45725: F, t45729: F, t45731: F, t45735: F, t45736: F, t45737: F, t45741: F, t45744: F, t45747: F, t45753: F, t549: F, t6066: F, t7630: F, t8793: F) -> F {
    let t45755 = F::new(0.35750489951850426669e0) * t9823 * t11765;
    let t45758 = t2536 * t3614;
    let t45761 = F::new(0.35750489951850426669e0) * t2021 * t45758 * t2009;
    let t45766 = F::new(0.87421871174939309262e2) * t2684 * t7585 * t45320;
    let t45767 = -F::new(0.79445533226334281487e-1) * t37166 * t549 * t44879 - F::new(0.12780975317973583226e1) * t45723 - F::new(0.38342925953920749677e0) * t45725 + F::new(0.85206502119823888171e-1) * t45729 - t45731 - t45735 + t45736 + t45737 - F::new(0.76685851907841499353e0) * t43919 - F::new(0.76685851907841499353e0) * t43922 + F::new(0.51123901271894332903e0) * t45741 + t45744 - t45747 - F::new(0.14300195980740170668e1) * t7630 * t6066 * t45337 - t45753 + t45755 + F::new(0.21450293971110256002e1) * t8793 * t10984 - t45761 + F::new(0.71500979903700853338e0) * t10886 * t11001 + t45766;
    t45767
}
