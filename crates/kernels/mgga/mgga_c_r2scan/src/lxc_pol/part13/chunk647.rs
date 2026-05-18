//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 647/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk647<F: Float>(t322: F, t3357: F, t3368: F, t3625: F, t3627: F, t3630: F) -> (F, F) {
    let t324 = F::new(0.0) < t322;
    let t3632 = t3357 + t3625 / F::new(8.0) - t3627 / F::new(8.0) + t3630 / F::new(4.0) + t3368;
    let t3633 = piecewise3::<f64>(t324, F::new(0.0), t3632);
    (t3632, t3633)
}
