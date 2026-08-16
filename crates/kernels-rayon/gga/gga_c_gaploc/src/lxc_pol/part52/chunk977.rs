//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 977/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk977(t14373: f64, t2009: f64, t2684: f64, t45737: f64, t45741: f64, t45744: f64, t45747: f64, t45753: f64, t45755: f64, t45761: f64, t45766: f64, t45772: f64, t45775: f64, t45778: f64, t45785: f64, t45792: f64, t45793: f64, t45794: f64, t45795: f64, t45798: f64, t45801: f64, t50130: f64, t7585: f64, t773: f64) -> f64 {
    let t50263 = t45737 + 0.87421871174939309263e2_f64 * t2684 * t7585 * t50130 + 0.51123901271894332901e0_f64 * t45741 + t45744 - t45747 - t45753 + t45755 - t45761 + t45766 - t45772 + t45775 - t45778 + t45785 - 0.35750489951850426669e0_f64 * t773 * t14373 * t2009 - t45792 - t45793 - t45794 - t45795 + t45798 + t45801;
    t50263
}
