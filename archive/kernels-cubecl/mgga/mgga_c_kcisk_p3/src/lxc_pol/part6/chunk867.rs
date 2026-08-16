//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 867/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk867<F: Float>(t10642: F, t28371: F, t28375: F, t28383: F, t28391: F, t28410: F, t28412: F, t28415: F, t28417: F, t28420: F, t28423: F, t28426: F, t28431: F, t28435: F) -> F {
    let t28506 = -F::cast_from(0.82785e-1_f64) * t28410 - F::cast_from(0.3883875e1_f64) * t28412 - t10642 - F::cast_from(0.412621875e-1_f64) * t28415 + F::cast_from(0.19419375e1_f64) * t28417 - F::cast_from(0.36793333333333333333e-1_f64) * t28420 - F::cast_from(0.82785e-1_f64) * t28423 - F::cast_from(0.49671e0_f64) * t28426 + F::cast_from(0.12077e1_f64) * t28375 - F::cast_from(0.181155e1_f64) * t28383 + F::cast_from(0.16557e0_f64) * t28431 - F::cast_from(0.33547222222222222222e0_f64) * t28371 - F::cast_from(0.301925e0_f64) * t28391 + F::cast_from(0.16504875e0_f64) * t28435;
    t28506
}
