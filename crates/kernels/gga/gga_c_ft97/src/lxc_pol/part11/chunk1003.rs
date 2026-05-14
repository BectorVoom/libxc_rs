//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1003/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1003<F: Float>(t10409: F, t43938: F, t446: F, t43511: F, t43516: F, t43519: F, t43522: F, t43528: F, t43531: F, t43534: F, t43538: F, t43541: F, t43551: F, t43926: F, t43930: F, t43933: F, t43936: F) -> (F, F) {
    let t43940 = t446 * t10409 * t43938;
    let t43942 = -8.0 / 3.0 * t43511 + 2.0 * t43516 + 16.0 / 9.0 * t43519 + 8.0 / 3.0 * t43522 + 8.0 * t43528 + 4.0 / 9.0 * t43531 - 16.0 / 81.0 * t43534 + t43538 - t43541 / 4.0 - 5.0 / 16.0 * t43551 + t43926 / 6.0 - t43930 / 3.0 + 112.0 / 81.0 * t43933 - 8.0 / 9.0 * t43936 + 8.0 / 9.0 * t43940;
    (t43940, t43942)
}
