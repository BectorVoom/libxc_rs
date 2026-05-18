//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 186/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk186<F: Float>(t44: F, t51: F, t415: F, t99: F, t101: F, t419: F, zeta_threshold: F) -> F {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t555 = piecewise3::<f64>(t45, F::new(0.0), F::new(5.0) / F::new(3.0) * t99 * t415);
    let t558 = piecewise3::<f64>(t52, F::new(0.0), F::new(5.0) / F::new(3.0) * t101 * t419);
    let t560 = t555 / F::new(2.0) + t558 / F::new(2.0);
    t560
}
