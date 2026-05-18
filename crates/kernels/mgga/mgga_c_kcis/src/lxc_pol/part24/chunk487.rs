//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 487/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk487<F: Float>(t956: F, t962: F, t265: F, t3005: F, t3031: F, t187: F, t426: F) -> (F, F, F, F, F, F) {
    let t3582 = t956 * t962;
    let t3585 = t265 * t3005;
    let t3592 = t265 * t3031;
    let t3600 = t187 * t956;
    let t3621 = t426 * t426;
    let t3622 = F::new(1.0) / t3621;
    (t3582, t3585, t3592, t3600, t3621, t3622)
}
