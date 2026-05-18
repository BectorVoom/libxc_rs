//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 824/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk824<F: Float>(t108: F, t8633: F, t8639: F, t8645: F, t8659: F, t915: F, t95: F, t2892: F, t5052: F, t481: F, t2505: F, t2526: F) -> (F, F, F, F) {
    let t8662 = (t8633 + t8639 + t8645 + t8659) * t108;
    let t8668 = t915 * t95;
    let t8675 = t5052 * t2892;
    let t8676 = t8675 * t481;
    let t8679 = t2505 * t2526;
    (t8662, t8668, t8676, t8679)
}
