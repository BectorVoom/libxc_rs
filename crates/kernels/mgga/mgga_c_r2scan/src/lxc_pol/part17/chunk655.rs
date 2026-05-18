//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 655/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk655<F: Float>(t32: F, t4715: F, t5: F, t1449: F, t68: F, t63: F, t1435: F, t437: F, t1453: F, t1683: F, t3: F, t40: F) -> (F, F, F, F, F) {
    let t4720 = t5 * t4715 * t32;
    let t4721 = F::new(0.34450798614814814813e-2) * t4720;
    let t4726 = F::new(1.0) / t1449 / t68;
    let t4727 = t63 * t4726;
    let t4728 = t1435 * t437;
    let t4729 = t4728 * t1453;
    let t4732 = t1683 * t3;
    let t4733 = t4732 * t40;
    (t4721, t4727, t4728, t4729, t4733)
}
