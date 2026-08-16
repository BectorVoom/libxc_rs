//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1358/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1358<F: Float>(t1484: F, t23097: F, t5617: F, t815: F, t1509: F, t20986: F, t2628: F, t6605: F, t20969: F, t6614: F, t1512: F, t98684: F) -> (F, F, F, F) {
    let t105329 = t23097 * t815 * t5617 * t1484;
    let t105333 = t6605 * t2628 * t20986 * t1509;
    let t105335 = t6614 * t20969;
    let t105337 = t98684 * t1512;
    (t105329, t105333, t105335, t105337)
}
