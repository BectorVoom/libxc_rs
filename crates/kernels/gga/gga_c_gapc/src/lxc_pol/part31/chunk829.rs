//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 829/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk829<F: Float>(t9384: F, t9391: F, t9393: F, t9397: F, t9400: F, t9403: F, t9406: F, t9409: F, t9412: F, t9416: F, t9420: F, t9423: F, t9426: F) -> F {
    let t9428 = -F::cast_from(0.14480154210752868924e-5_f64) * t9384 + F::cast_from(0.19738380876484260726e-4_f64) * t9391 - F::cast_from(0.2318836277704281739e-4_f64) * t9393 - F::cast_from(0.14758978949652777778e-5_f64) * t9397 - F::cast_from(0.89947526170248609072e-8_f64) * t9400 + F::cast_from(0.1199450261480265202e-7_f64) * t9403 - F::cast_from(0.86880925264517213544e-4_f64) * t9406 - F::cast_from(0.4637672555408563478e-4_f64) * t9409 - F::cast_from(0.38647271295071362318e-6_f64) * t9412 + F::cast_from(0.1667571362635586049e-8_f64) * t9416 + F::cast_from(0.8206551981474340792e-8_f64) * t9420 + F::cast_from(0.10120442708333333334e-4_f64) * t9423 - F::cast_from(0.50602213541666666669e-5_f64) * t9426;
    t9428
}
