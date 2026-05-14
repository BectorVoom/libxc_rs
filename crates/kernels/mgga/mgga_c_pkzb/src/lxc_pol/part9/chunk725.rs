//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 725/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk725<F: Float>(t12: F, t24: F, t1692: F, t192: F, t1646: F, t1837: F, t207: F, t5094: F, t5100: F, t653: F, t1655: F, t2179: F, t333: F, t5107: F, t5113: F, t822: F, zeta_threshold: F) -> (F, F) {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t5196 = t192 * t1692;
    let t5207 = piecewise3(t84, 0.0, 8.0 / 27.0 * t1837 * t5094 - 2.0 / 3.0 * t653 * t1646 + 2.0 / 3.0 * t207 * t5100);
    let t5215 = piecewise3(t90, 0.0, 8.0 / 27.0 * t2179 * t5107 - 2.0 / 3.0 * t822 * t1655 + 2.0 / 3.0 * t333 * t5113);
    let t5217 = t5207 / 2.0 + t5215 / 2.0;
    (t5196, t5217)
}
