//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1212/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1212<F: Float>(t237: F, t8040: F, t2380: F, t6475: F, t8474: F, t8345: F, t1220: F, t6377: F, t3235: F, t3237: F, t5939: F, t179: F, t3026: F, t404: F, t6380: F, t1184: F, t2240: F) -> (F, F, F, F, F, F, F) {
    let t22394 = t237 * t8040;
    let t22445 = t2380 * t6475 * t8474;
    let t22452 = t2380 * t6475 * t8345;
    let t22461 = t1220 * t6377;
    let t22469 = t3235 * t5939 * t3237;
    let t22474 = t404 * t179 * t6380 * t3026;
    let t22500 = t2240 * t1184;
    (t22394, t22445, t22452, t22461, t22469, t22474, t22500)
}
