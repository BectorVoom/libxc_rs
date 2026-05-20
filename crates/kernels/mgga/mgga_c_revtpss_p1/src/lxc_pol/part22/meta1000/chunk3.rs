//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3401/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3401<F: Float>(t1100: F, t5019: F, t18898: F, t41813: F, t981: F, t19023: F, t3022: F, t41520: F, t51967: F, t63274: F, t63276: F, t63278: F, t63281: F, t63285: F, t63290: F, t63293: F, t63299: F, t63304: F, t63308: F) -> (F, F, F, F) {
    let t63827 = t1100 * t5019;
    let t63833 = F::cast_from(0.10254018858216406658e4_f64) * t981 * t18898 * t41813;
    let t63835 = F::cast_from(0.11696447245269292414e1_f64) * t3022 * t19023;
    let t63847 = F::cast_from(0.37083333333333333334e-1_f64) * t63274 - F::cast_from(0.12361111111111111111e-1_f64) * t63276 + F::cast_from(0.41203703703703703704e-2_f64) * t63278 - F::cast_from(0.12361111111111111111e-1_f64) * t63281 - F::cast_from(0.61805555555555555555e-2_f64) * t63285 - F::cast_from(0.10300925925925925926e-1_f64) * t63290 + F::cast_from(0.37083333333333333334e-1_f64) * t63293 + F::cast_from(0.18541666666666666667e-1_f64) * t63299 + F::cast_from(0.12361111111111111111e0_f64) * t63304 - F::cast_from(0.22249999999999999999e0_f64) * t63308 + t41520 + F::cast_from(0.61805555555555555556e-2_f64) * t51967;
    (t63827, t63833, t63835, t63847)
}
