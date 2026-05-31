//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 859/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk859<F: Float>(t10649: F, t15989: F, t22564: F, t22575: F, t22583: F, t28371: F, t28375: F, t28379: F, t28383: F, t28387: F, t28391: F, t1646: F) -> (F, F) {
    let t28393 = -t10649 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t15989 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t22564 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t22575 + t22583 / F::cast_from(3.0_f64) - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t28371 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t28375 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28379 - F::cast_from(2.0_f64) * t28383 + F::cast_from(2.0_f64) * t28387 - t28391 / F::cast_from(3.0_f64);
    let t28394 = t1646 * t28393;
    (t28393, t28394)
}
