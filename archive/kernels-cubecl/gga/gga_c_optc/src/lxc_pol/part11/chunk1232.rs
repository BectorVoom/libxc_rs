//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1232/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1232<F: Float>(t43: F, t50: F, t38346: F, t13064: F, t16231: F, t1884: F, t22308: F, t3365: F, t4565: F, t47: F, t55901: F, t55906: F, t55912: F, t13076: F, t16241: F, t1896: F, t22323: F, t3373: F, t4573: F, t52: F, t55917: F, t55922: F, t55927: F, zeta_threshold: F) -> (F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t56263 = F::cast_from(0.70178680769462448852e1_f64) * t38346;
    let t56275 = piecewise3::<F>(t44, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t22308 * t55901 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t13064 * t4565 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1884 * t55906 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t3365 * t16231 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t47 * t55912);
    let t56287 = piecewise3::<F>(t51, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t22323 * t55917 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t13076 * t4573 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1896 * t55922 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t3373 * t16241 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t52 * t55927);
    (t56263, t56275, t56287)
}
