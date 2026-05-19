//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 868/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk868<F: Float>(t28492: F, t28506: F, t1676: F, t1685: F, t10569: F, t15989: F, t22564: F, t22575: F, t22583: F, t28371: F, t28375: F, t28379: F, t28383: F, t28387: F, t28391: F) -> (F, F, F) {
    let t28507 = t28492 + t28506;
    let t28509 = t1676 * t28507 * t1685;
    let t28528 = -t10569 - F::cast_from(0.23744444444444444444e-1_f64) * t15989 + F::cast_from(0.11872222222222222222e-1_f64) * t22564 - F::cast_from(0.35616666666666666666e-1_f64) * t22575 + F::cast_from(0.17808333333333333333e-1_f64) * t22583 - F::cast_from(0.19787037037037037037e-1_f64) * t28371 + F::cast_from(0.71233333333333333332e-1_f64) * t28375 - F::cast_from(0.35616666666666666666e-1_f64) * t28379 - F::new(0.10685e0) * t28383 + F::new(0.10685e0) * t28387 - F::cast_from(0.17808333333333333333e-1_f64) * t28391;
    (t28507, t28509, t28528)
}
