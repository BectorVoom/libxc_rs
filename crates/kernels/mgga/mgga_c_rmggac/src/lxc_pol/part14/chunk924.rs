//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 924/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk924<F: Float>(t7244: F, t9171: F, t1528: F, t1970: F, t209: F, t236: F, t476: F, t7231: F, t7255: F, t9153: F, t1587: F, t3352: F) -> (F, F, F, F) {
    let t39926 = t7244 * t9171;
    let t39927 = F::cast_from(0.19863479950205658386e-4_f64) * t39926;
    let t39932 = t1970 * t7231 * t236 * t1528 * t476 * t209;
    let t39934 = t7255 * t9153;
    let t39940 = t1970 * t3352 * t236 * t1587 * t476 * t209;
    (t39927, t39932, t39934, t39940)
}
