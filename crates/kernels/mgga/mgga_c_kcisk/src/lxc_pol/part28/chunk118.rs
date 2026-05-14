//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 118/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk118<F: Float>(t606: F, t609: F, t612: F, t615: F) -> (F, F, F, F) {
    let t625 = 1.0 + 0.278125e-1 * t606;
    let t630 = 0.51785e1 * t609 + 0.905775e0 * t606 + 0.1100325e0 * t612 + 0.248355e0 * t615;
    let t633 = 1.0 + 0.29608574643216675549e2 / t630;
    let t634 = f64::ln(t633);
    (t625, t630, t633, t634)
}
