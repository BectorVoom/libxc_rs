//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1283/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1283<F: Float>(t119935: F, t30262: F, t375: F, t89: F, t1039: F, t27157: F, t3526: F, t574: F, t5900: F, t4778: F, t590: F, t105592: F, t4753: F, t23649: F, t30208: F, t30270: F, t376: F) -> (F, F, F, F, F, F, F, F, F) {
    let t119936 = 2.0 * t119935;
    let t119938 = t89 * t375 * t30262;
    let t119939 = t119938 / 3.0;
    let t119943 = t27157 * t574 * t5900 * t1039 * t3526;
    let t119948 = t27157 * t574 * t5900 * t4778 * t590;
    let t119953 = t105592 * t574 * t5900 * t4753 * t590;
    let t119955 = t23649 * t30208;
    let t119956 = t119955 / 9.0;
    let t119959 = t89 * t376 * t30270;
    (t119936, t119938, t119939, t119943, t119948, t119953, t119955, t119956, t119959)
}
