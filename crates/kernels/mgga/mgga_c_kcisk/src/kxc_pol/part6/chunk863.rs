//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 863/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk863<F: Float>(t28408: F, t28437: F, t1664: F, t1645: F, t10757: F, t28357: F, t10755: F, t10761: F, t15989: F, t22564: F, t22575: F, t22583: F, t28371: F, t28375: F, t28379: F, t28383: F, t28387: F, t28391: F) -> (F, F, F) {
    let t28438 = t28408 + t28437;
    let t28439 = t28438 * t1664;
    let t28441 = F::new(1.0) * t1645 * t28439;
    let t28442 = t28357 * t10757;
    let t28444 = F::cast_from(0.51725014705706168417e3_f64) * t10755 * t28442;
    let t28455 = -t10761 - F::cast_from(0.12361111111111111111e-1_f64) * t15989 + F::cast_from(0.61805555555555555556e-2_f64) * t22564 - F::cast_from(0.18541666666666666667e-1_f64) * t22575 + F::cast_from(0.92708333333333333334e-2_f64) * t22583 - F::cast_from(0.10300925925925925926e-1_f64) * t28371 + F::cast_from(0.37083333333333333333e-1_f64) * t28375 - F::cast_from(0.18541666666666666666e-1_f64) * t28379 - F::cast_from(0.55625000000000000001e-1_f64) * t28383 + F::cast_from(0.55625000000000000001e-1_f64) * t28387 - F::cast_from(0.92708333333333333333e-2_f64) * t28391;
    (t28441, t28444, t28455)
}
