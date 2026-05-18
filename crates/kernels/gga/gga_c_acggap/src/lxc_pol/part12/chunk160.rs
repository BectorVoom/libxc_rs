//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 160/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk160<F: Float>(t43: F, t47: F, t474: F, zeta_threshold: F) -> (F, F) {
    let t44 = t43 <= zeta_threshold;
    let t477 = piecewise3::<f64>(t44, F::new(0.0), F::new(4.0) / F::new(3.0) * t47 * t474);
    let t478 = -t474;
    (t477, t478)
}
