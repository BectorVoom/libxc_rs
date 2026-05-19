//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 706/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk706<F: Float>(t2100: F, t7538: F, t7463: F, t7465: F, t7466: F, t7469: F, t7473: F, t7479: F, t7481: F, t7485: F, t7489: F, t7491: F, t7497: F, t7500: F, t7504: F, t7516: F, t7520: F, t7524: F, t7529: F, t7531: F, t7536: F) -> (F, F) {
    let t7539 = t7538 * t2100;
    let t7540 = F::cast_from(0.31448092289604152068e-3_f64) * t7539;
    let t7541 = -t7463 + t7465 - F::cast_from(0.56606566121287473722e-2_f64) * t7466 + t7469 + F::cast_from(0.7862023072401038017e-3_f64) * t7473 + F::cast_from(0.10482697429868050689e-2_f64) * t7479 - F::cast_from(0.62896184579208304136e-3_f64) * t7481 + t7485 + t7489 + F::new(0.1528125e-1) * t7491 - t7497 + F::cast_from(0.62896184579208304136e-3_f64) * t7500 - F::new(0.4584375e-1) * t7504 + t7516 - t7520 - F::cast_from(0.31448092289604152068e-3_f64) * t7524 - F::cast_from(0.41930789719472202757e-3_f64) * t7529 + F::cast_from(0.94344276868812456204e-3_f64) * t7531 + F::cast_from(0.47172138434406228102e-3_f64) * t7536 + t7540;
    (t7540, t7541)
}
