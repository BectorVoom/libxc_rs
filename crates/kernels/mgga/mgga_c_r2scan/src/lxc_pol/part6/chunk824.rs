//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 824/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk824<F: Float>(t5511: F, t5562: F, t5610: F, t5647: F, t5702: F, t5744: F, t5797: F, t5841: F, t61: F, t41: F, t1419: F, t661: F, t5409: F, t5411: F, t5413: F, t5433: F, t5437: F, t5441: F, t5444: F, t5451: F, t5454: F, t5459: F, t5463: F, t5467: F, t5470: F, t5474: F, t5475: F, t5479: F) -> (F, F, F, F, F) {
    let t5844 = t5511 + t5562 + t5610 + t5647 + t5702 + t5744 + t5797 + t5841;
    let t5845 = t61 * t5844;
    let t5846 = t41 * t5845;
    let t5847 = t1419 * t661;
    let t5849 = 0.127022098e-2 * t5409 + 0.17544670867903938621e1 * t5411 + 0.17544670867903938621e1 * t5413 + t5433 - t5437 + t5441 + t5444 + t5451 + t5454 - t5459 + t5463 + t5467 + 0.254044196e-2 * t5470 + t5474 - t41 * t5475 - t5479 - t5846 - 36.0 * t5847;
    (t5844, t5845, t5846, t5847, t5849)
}
