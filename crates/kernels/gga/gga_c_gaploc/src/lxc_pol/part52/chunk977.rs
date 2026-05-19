//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 977/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk977<F: Float>(t14373: F, t2009: F, t2684: F, t45737: F, t45741: F, t45744: F, t45747: F, t45753: F, t45755: F, t45761: F, t45766: F, t45772: F, t45775: F, t45778: F, t45785: F, t45792: F, t45793: F, t45794: F, t45795: F, t45798: F, t45801: F, t50130: F, t7585: F, t773: F) -> F {
    let t50263 = t45737 + F::cast_from(0.87421871174939309263e2_f64) * t2684 * t7585 * t50130 + F::cast_from(0.51123901271894332901e0_f64) * t45741 + t45744 - t45747 - t45753 + t45755 - t45761 + t45766 - t45772 + t45775 - t45778 + t45785 - F::cast_from(0.35750489951850426669e0_f64) * t773 * t14373 * t2009 - t45792 - t45793 - t45794 - t45795 + t45798 + t45801;
    t50263
}
