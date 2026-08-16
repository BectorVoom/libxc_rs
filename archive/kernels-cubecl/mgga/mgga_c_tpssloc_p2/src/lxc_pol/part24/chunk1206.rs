//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1206/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1206<F: Float>(t6688: F, t974: F, t23631: F, t381: F, t883: F, t6743: F, t14227: F, t6800: F, t23384: F, t6790: F, t1949: F, t3010: F) -> (F, F, F, F, F, F, F, F) {
    let t23632 = t974 * t6688;
    let t23633 = t23631 * t23632;
    let t23634 = t381 * t883;
    let t23635 = t6743 * t23634;
    let t23636 = t14227 * t6800;
    let t23637 = t23635 * t23636;
    let t23642 = t23384 * t6790;
    let t23644 = t3010 * t1949;
    (t23632, t23633, t23634, t23635, t23636, t23637, t23642, t23644)
}
