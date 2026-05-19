//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1246/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1246<F: Float>(t31259: F, t31297: F, t31318: F, t32760: F, t32763: F, t32765: F, t35506: F, t35507: F, t35508: F, t35527: F, t35529: F, t35538: F, t35539: F, t35541: F, t37583: F, t37584: F, t40029: F, t40034: F) -> F {
    let t41903 = F::cast_from(0.39221874999999999999e0_f64) * t31259 + t35506 - t35507 - t35508 + t37583 + t37584 - t32760 - F::cast_from(0.20579528696673473747e-1_f64) * t40029 - t35527 + t32763 - F::cast_from(0.13719685797782315831e-1_f64) * t35529 - t32765 - F::cast_from(0.31448092289604152069e-2_f64) * t31297 - F::cast_from(0.42874018118069736972e-2_f64) * t40034 - t35538 + t35539 + F::cast_from(0.11321313224257494745e-1_f64) * t31318 + t35541;
    t41903
}
