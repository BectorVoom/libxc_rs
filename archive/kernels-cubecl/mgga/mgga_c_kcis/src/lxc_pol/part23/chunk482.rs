//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 482/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk482<F: Float>(t486: F, t4000: F, t4001: F, t286: F, t3951: F) -> (F, F, F) {
    let t495 = F::cast_from(0.0_f64) < t486;
    let t4002 = t4000 * t4001;
    let t4003 = t286 * t4002;
    let t4007 = piecewise3::<F>(t495, t3951, -t3951);
    (t4002, t4003, t4007)
}
