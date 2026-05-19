//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 151/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk151<F: Float>(t51: F, t419: F, t476: F, t475: F, zeta_threshold: F) -> F {
    let t52 = t51 <= zeta_threshold;
    let t477 = t476 * t419;
    let t479 = piecewise3::<F>(t52, F::new(0.0), F::new(2.0) / F::new(3.0) * t477);
    let t481 = t475 / F::new(2.0) + t479 / F::new(2.0);
    t481
}
