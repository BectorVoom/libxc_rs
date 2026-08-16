//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1184/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1184<F: Float>(t40192: F, t40196: F, t12054: F, t9333: F, t12065: F, t2437: F, t41853: F, t41854: F, t41863: F, t41867: F, t41871: F, t41874: F, t41876: F, t41880: F) -> F {
    let t47925 = F::cast_from(0.38342925953920749677e0_f64) * t40192;
    let t47926 = F::cast_from(0.85206502119823888171e-1_f64) * t40196;
    let t47927 = t12054 * t9333;
    let t47934 = t2437 * t12065;
    let t47936 = t41853 - t41854 - t47925 + t47926 - F::cast_from(0.10725146985555128001e1_f64) * t47927 + F::cast_from(0.11502877786176224903e2_f64) * t41863 + F::cast_from(0.11502877786176224903e2_f64) * t41867 + F::cast_from(0.11502877786176224903e2_f64) * t41871 + t41874 + F::cast_from(0.69017266717057349418e1_f64) * t41876 - F::cast_from(0.21450293971110256001e1_f64) * t41880 + F::cast_from(0.35750489951850426669e0_f64) * t47934;
    t47936
}
