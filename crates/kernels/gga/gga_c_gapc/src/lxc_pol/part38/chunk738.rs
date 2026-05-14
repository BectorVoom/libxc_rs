//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 738/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk738<F: Float>(t2629: F, t9411: F, t1084: F, t8986: F, t2562: F, t2636: F, t8619: F, t3327: F, t7191: F, t2316: F, t2982: F, t3391: F, t2300: F, t3387: F, t9384: F, t9391: F, t9393: F, t9397: F, t9400: F, t9403: F, t9406: F, t9409: F) -> (F, F, F, F, F, F, F) {
    let t9412 = t9411 * t2629;
    let t9414 = t1084 * t8986;
    let t9415 = t2636 * t2562;
    let t9416 = t9414 * t9415;
    let t9418 = t1084 * t8619;
    let t9419 = t3327 * t7191;
    let t9420 = t9418 * t9419;
    let t9422 = t2982 * t2316;
    let t9423 = t3391 * t9422;
    let t9425 = t2982 * t2300;
    let t9426 = t3387 * t9425;
    let t9428 = -0.14480154210752868924e-5 * t9384 + 0.19738380876484260726e-4 * t9391 - 0.2318836277704281739e-4 * t9393 - 0.14758978949652777778e-5 * t9397 - 0.89947526170248609072e-8 * t9400 + 0.1199450261480265202e-7 * t9403 - 0.86880925264517213544e-4 * t9406 - 0.4637672555408563478e-4 * t9409 - 0.38647271295071362318e-6 * t9412 + 0.1667571362635586049e-8 * t9416 + 0.8206551981474340792e-8 * t9420 + 0.10120442708333333334e-4 * t9423 - 0.50602213541666666669e-5 * t9426;
    (t9414, t9415, t9418, t9419, t9422, t9425, t9428)
}
