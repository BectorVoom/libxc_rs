//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 800/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk800<F: Float>(t1368: F, t16503: F, t3369: F, t7448: F, t34761: F, t9159: F, t2318: F, t34975: F, t7482: F, t7244: F, t9171: F, t1528: F, t1970: F, t209: F, t236: F, t476: F, t7231: F) -> (F, F, F, F, F) {
    let t39915 = t16503 * t3369 * t1368 * t7448;
    let t39917 = t34761 * t9159;
    let t39921 = t34975 * t3369 * t2318 * t7482;
    let t39926 = t7244 * t9171;
    let t39932 = t1970 * t7231 * t236 * t1528 * t476 * t209;
    (t39915, t39917, t39921, t39926, t39932)
}
