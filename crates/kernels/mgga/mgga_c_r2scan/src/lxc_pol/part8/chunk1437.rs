//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1437/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1437<F: Float>(t2266: F, t2526: F, t9589: F, t19388: F, t19394: F, t19405: F, t23820: F, t32963: F, t32965: F, t32967: F, t34819: F, t34822: F, t34827: F, t34830: F, t8597: F, t2854: F, t2858: F, t9129: F) -> (F, F, F, F) {
    let t34833 = 9.0 * t2266 * t9589 * t2526;
    let t34834 = -t34819 + t19388 + t19394 + t32963 - t19405 - t34822 + 3.0 * t23820 - t32965 - t34827 + t34830 + t32967 + t34833;
    let t34837 = 9.0 * t2266 * t8597 * t2526;
    let t34840 = 36.0 * t2858 * t2854 * t9129;
    (t34833, t34834, t34837, t34840)
}
