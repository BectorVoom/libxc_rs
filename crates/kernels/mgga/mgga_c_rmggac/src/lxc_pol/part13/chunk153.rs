//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 153/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk153<F: Float>(t446: F, t489: F, t490: F, t467: F, t479: F, t487: F, t488: F) -> (F, F) {
    let t492 = t489 * t490 * t446;
    let t495 = -F::cast_from(0.27439556402611977244e-1_f64) * t467 * t479 - t487 - F::cast_from(0.54879112805223954488e-1_f64) * t488 * t492;
    (t492, t495)
}
