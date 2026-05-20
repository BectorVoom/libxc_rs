//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1812/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1812<F: Float>(t1179: F, t1188: F, t12547: F, t1196: F, t300: F, t3488: F) -> (F, F, F) {
    let t12564 = t1179 * t12547 * t1188;
    let t12566 = F::cast_from(0.5848223622634646207e0_f64) * t1196 * t12564;
    let t12571 = t300 * t3488;
    (t12564, t12566, t12571)
}
