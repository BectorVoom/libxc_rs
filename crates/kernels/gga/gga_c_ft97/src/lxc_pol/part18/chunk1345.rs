//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1345/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1345<F: Float>(t12001: F, t27044: F, t1651: F, t23657: F, t23671: F, t6656: F, t105786: F, t27142: F, t27181: F, t376: F, t89: F, t2185: F, t3526: F, t558: F, t5900: F, t9114: F) -> (F, F, F, F, F, F, F) {
    let t105884 = t12001 * t27044;
    let t105888 = t23657 * t23671 * t6656 * t1651;
    let t105891 = t27142 * t23671 * t105786;
    let t105894 = t89 * t376 * t27181;
    let t105895 = 4.0 / 3.0 * t105894;
    let t105899 = t23657 * t2185 * t5900 * t3526 * t558;
    let t105900 = t9114 * t5900;
    (t105884, t105888, t105891, t105894, t105895, t105899, t105900)
}
