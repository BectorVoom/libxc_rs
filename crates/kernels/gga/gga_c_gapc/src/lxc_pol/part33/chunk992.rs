//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 992/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk992<F: Float>(t11892: F, t3368: F, t11473: F, t3322: F, t3363: F, t3330: F, t33560: F, t1971: F, t8785: F, t1084: F, t29571: F, t1461: F, t8709: F, t28517: F, t1044: F, t825: F) -> (F, F, F, F, F, F, F, F) {
    let t34066 = t11892 * t3368;
    let t34069 = t3363 * t11473 * t3322;
    let t34071 = t33560 * t3330;
    let t34073 = t1971 * t8785;
    let t34075 = t1084 * t34073 * t29571;
    let t34077 = t1461 * t8709;
    let t34079 = t1084 * t34077 * t28517;
    let t34081 = t825 * t1044;
    (t34066, t34069, t34071, t34073, t34075, t34077, t34079, t34081)
}
