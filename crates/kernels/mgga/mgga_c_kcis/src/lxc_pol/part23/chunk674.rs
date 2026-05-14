//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 674/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk674<F: Float>(t1395: F, t2062: F, t2066: F, t8189: F, t8192: F, t8194: F, t8197: F, t8199: F, t8201: F) -> (F, F, F) {
    let t8203 = t1395 * t2062;
    let t8205 = t1395 * t2066;
    let t8207 = t8189 / 16.0 - t8192 / 16.0 - t8194 / 6.0 + t8197 / 24.0 - t8199 / 128.0 + t8201 / 128.0 + t8203 / 24.0 - t8205 / 96.0;
    (t8203, t8205, t8207)
}
