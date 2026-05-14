//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 756/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk756<F: Float>(t109: F, t111: F, t1536: F, t1544: F, t1547: F, t2498: F, t2504: F, t2506: F, t2527: F, t486: F, t491: F, t7165: F, t7175: F, t7181: F, t7185: F, t7188: F, t7191: F, t915: F, t917: F) -> (F,) {
    let t7194 = 3.0 * t109 * t7191 - t7165 * t111 + 3.0 * t1536 * t917 - 12.0 * t915 * t1544 + 3.0 * t915 * t1547 + 6.0 * t2498 * t491 + 60.0 * t2504 * t7181 - 24.0 * t2504 * t7185 - 12.0 * t2504 * t7188 - 24.0 * t7175 * t2506 + 6.0 * t486 * t2527;
    (t7194,)
}
