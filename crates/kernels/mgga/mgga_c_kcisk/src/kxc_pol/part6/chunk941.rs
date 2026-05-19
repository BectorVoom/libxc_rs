//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 941/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk941<F: Float>(t12043: F, t28371: F, t28375: F, t28383: F, t28391: F, t28410: F, t28412: F, t28415: F, t28417: F, t28420: F, t28423: F, t28426: F, t28431: F, t28435: F) -> F {
    let t29667 = -F::new(0.104195e0) * t28410 - F::new(0.52945875e1) * t28412 - t12043 - F::cast_from(0.157790625e0_f64) * t28415 + F::cast_from(0.264729375e1_f64) * t28417 - F::cast_from(0.46308888888888888889e-1_f64) * t28420 - F::new(0.104195e0) * t28423 - F::new(0.62517e0) * t28426 + F::new(0.20659e1) * t28375 - F::new(0.309885e1) * t28383 + F::new(0.20839e0) * t28431 - F::cast_from(0.57386111111111111112e0_f64) * t28371 - F::new(0.516475e0) * t28391 + F::new(0.6311625e0) * t28435;
    t29667
}
