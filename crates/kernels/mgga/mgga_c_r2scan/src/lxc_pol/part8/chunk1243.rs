//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1243/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1243<F: Float>(t2133: F, t2563: F, t6848: F, t2182: F, t25214: F, t6091: F, t2562: F, t264: F, t7155: F, t1527: F, t8908: F, t2294: F, t2598: F, t9547: F, t7494: F, t9526: F) -> (F, F, F, F, F, F, F) {
    let t27245 = t2133 * t6848 * t2563;
    let t27246 = 0.12713391885412927226e1 * t27245;
    let t27256 = t2182 * t6091 * t25214;
    let t27257 = t264 * t2562;
    let t27461 = 0.35089341735807877242e1 * t7155;
    let t27475 = t8908 * t1527;
    let t27644 = t2598 * t2294 * t9547;
    let t27650 = t7494 * t9526;
    (t27246, t27256, t27257, t27461, t27475, t27644, t27650)
}
