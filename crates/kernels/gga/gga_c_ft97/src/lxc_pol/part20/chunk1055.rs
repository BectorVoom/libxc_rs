//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1055/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1055<F: Float>(t108016: F, t24438: F, t6118: F, t14075: F, t24531: F, t13863: F, t96945: F, t27762: F, t14116: F, t27805: F, t107997: F, t107999: F, t108001: F, t108003: F, t108006: F, t108010: F, t108014: F) -> (F, F, F, F, F, F, F, F) {
    let t108018 = t6118 * t24438 * t108016;
    let t108020 = t24531 * t14075;
    let t108022 = t6118 * t24438 * t108020;
    let t108024 = t96945 * t13863;
    let t108026 = t6118 * t27762 * t108024;
    let t108028 = t24531 * t14116;
    let t108030 = t27805 * t24438 * t108028;
    let t108032 = t107997 + t107999 - t108001 + t108003 - 2.0 / 3.0 * t108006 - t108010 / 3.0 + 4.0 / 3.0 * t108014 - 2.0 / 3.0 * t108018 - t108022 / 3.0 - 2.0 / 3.0 * t108026 + 4.0 / 3.0 * t108030;
    (t108018, t108020, t108022, t108024, t108026, t108028, t108030, t108032)
}
