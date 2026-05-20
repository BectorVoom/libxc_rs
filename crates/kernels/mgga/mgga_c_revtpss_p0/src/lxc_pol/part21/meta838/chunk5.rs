//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3144/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3144<F: Float>(t12429: F, t1744: F, t12423: F, t12430: F, t12464: F, t12508: F, t12511: F, t16951: F, t16955: F, t16958: F, t16962: F, t16965: F, t16966: F, t1745: F, t3452: F, t3453: F, t3471: F, t45080: F, t45085: F, t45197: F, t5143: F, t56279: F, t56281: F, t56283: F, t56286: F, t56290: F, t57799: F) -> F {
    let t57944 = t12429 * t1744;
    let t57967 = -t56279 + t56281 - t56283 + t56286 - t56290 - F::cast_from(0.57895126195293126243e3_f64) * t57944 * t12508 - t57799 - F::new(6.0) * t12511 * t16951 - F::cast_from(0.57895126195293126242e3_f64) * t45197 * t16955 + F::cast_from(0.96491876992155210402e2_f64) * t12423 * t16962 + F::cast_from(0.6207121550312808036e4_f64) * t45080 * t16966 - F::new(6.0) * t3452 * t5143 * t3471 - F::cast_from(0.57895126195293126242e3_f64) * t12429 * t16958 * t3453 - F::new(2.0) * t3452 * t1745 * t12464 - F::cast_from(0.24828486201251232145e5_f64) * t45085 * t16965 * t12430;
    t57967
}
