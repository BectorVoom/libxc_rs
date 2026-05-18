//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 99/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk99<F: Float>(t453: F, t456: F, t459: F, t463: F) -> (F, F, F) {
    let t478 = F::new(0.51785e1) * t456 + F::new(0.905775e0) * t453 + F::new(0.1100325e0) * t459 + F::new(0.1241775e0) * t463;
    let t481 = F::new(1.0) + F::new(0.29608574643216675549e2) / t478;
    let t482 = f64::ln(t481);
    (t478, t481, t482)
}
