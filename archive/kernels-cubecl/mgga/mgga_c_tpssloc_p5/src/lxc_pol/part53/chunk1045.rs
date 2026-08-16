//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1045/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1045<F: Float>(t32244: F, t45844: F, t12571: F, t116935: F, t33107: F, t116919: F, t33119: F, t32248: F, t116909: F, t33111: F, t116905: F, t116932: F, t116942: F, t116947: F, t116954: F, t119884: F, t119892: F, t119909: F, t119955: F, t119971: F, t119975: F, t119990: F, t31006: F, t31013: F, t31024: F, t32245: F, t32258: F, t8707: F) -> F {
    let t124335 = t45844 * t32244;
    let t124338 = t12571 * t32244;
    let t124351 = t116935 * t33107;
    let t124353 = t116919 * t33119;
    let t124355 = t12571 * t32248;
    let t124360 = t116909 * t33111;
    let t124364 = F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t116905 * t119884 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t116954 * t119892 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t124335 * t31006 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t124338 * t31024 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t116942 * t33107 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t32245 * t119990 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t116947 * t33119 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t32258 * t119971 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t32258 * t119975 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t124351 - F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t124353 - F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t124355 * t31013 + F::cast_from(35.0_f64) / F::cast_from(6.0_f64) * t116932 * t119909 + F::cast_from(80.0_f64) / F::cast_from(27.0_f64) * t124360 + F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t119955 * t8707;
    t124364
}
