//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1252/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1252<F: Float>(t108178: F, t1154: F, t2347: F, t3886: F, t96934: F, t1091: F, t27814: F, t96935: F, t124007: F, t27762: F, t27805: F, t108212: F, t27819: F, t6878: F, t109390: F, t24437: F, t3837: F) -> (F, F, F, F, F) {
    let t124054 = t96934 * t108178 * t1154 * t2347 * t3886;
    let t124058 = t96934 * t96935 * t1091 * t27814;
    let t124061 = t27805 * t27762 * t124007;
    let t124065 = t27819 * t108212 * t6878 * t27814;
    let t124069 = t24437 * t109390 * t6878 * t3837;
    (t124054, t124058, t124061, t124065, t124069)
}
