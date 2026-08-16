//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 860/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk860<F: Float>(t1659: F, t28385: F, t26: F, t10738: F, t15989: F, t16389: F, t22564: F, t22575: F, t22583: F, t22698: F, t22705: F, t22707: F, t28362: F, t28379: F, t28387: F, t28394: F) -> (F, F) {
    let t28403 = t1659 * t28385;
    let t28404 = t26 * t28403;
    let t28408 = -F::cast_from(0.39862222222222222223e0_f64) * t15989 + F::cast_from(0.46074375e0_f64) * t28362 + F::cast_from(0.1898925e1_f64) * t28394 - t10738 - F::cast_from(0.27385555555555555556e0_f64) * t16389 + F::cast_from(0.5477111111111111111e-1_f64) * t22698 + F::cast_from(0.19931111111111111111e0_f64) * t22564 - F::cast_from(0.59793333333333333333e0_f64) * t22575 + F::cast_from(0.29896666666666666667e0_f64) * t22583 - F::cast_from(0.32862666666666666666e0_f64) * t22705 + F::cast_from(0.16431333333333333333e0_f64) * t22707 + F::cast_from(0.49293999999999999999e0_f64) * t28404 - F::cast_from(0.59793333333333333333e0_f64) * t28379 + F::cast_from(0.17938e1_f64) * t28387;
    (t28404, t28408)
}
