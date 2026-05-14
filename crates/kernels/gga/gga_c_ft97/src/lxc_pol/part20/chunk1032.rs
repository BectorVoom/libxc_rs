//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1032/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1032<F: Float>(t24914: F, t8392: F, t24870: F, t25287: F, t24895: F, t24941: F, t6284: F, t8232: F, t6293: F, t24937: F, t24875: F, t24879: F, t25271: F, t56110: F, t1882: F, t25239: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t98904 = t8392 * t24914;
    let t98906 = t8392 * t24870;
    let t98924 = t8392 * t25287;
    let t98926 = t8392 * t24895;
    let t98933 = t8392 * t24941;
    let t98940 = t8232 * t6284;
    let t98942 = t8232 * t6293;
    let t98944 = t8392 * t24937;
    let t98957 = t8392 * t24875;
    let t98960 = t8392 * t24879;
    let t98966 = t56110 * t25271;
    let t99009 = t1882 * t25239;
    (t98904, t98906, t98924, t98926, t98933, t98940, t98942, t98944, t98957, t98960, t98966, t99009)
}
