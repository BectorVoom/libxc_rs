//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1297/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1297<F: Float>(t20275: F, t5483: F, t1675: F, t19380: F, t5790: F, t19345: F, t18350: F, t5492: F, t19396: F, t5791: F, t18646: F, t6073: F) -> (F, F, F, F, F, F, F) {
    let t67451 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t5483 * t20275;
    let t67454 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t1675 * t5790 * t19380;
    let t67472 = t5790 * t19345;
    let t67474 = F::cast_from(160.0_f64) / F::cast_from(9.0_f64) * t18350 * t67472;
    let t67480 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t5492 * t20275;
    let t67491 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t19396 * t5791;
    let t67496 = t6073 * t18646;
    (t67451, t67454, t67472, t67474, t67480, t67491, t67496)
}
