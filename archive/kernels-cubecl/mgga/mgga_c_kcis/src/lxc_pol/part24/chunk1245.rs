//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1245/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1245<F: Float>(t1262: F, t6272: F, t1267: F, t5310: F, t92651: F, t11081: F, t26960: F, t29116: F, t1092: F, t1121: F, t27763: F, t6696: F) -> (F, F, F, F, F) {
    let t100314 = t6272 * t1262;
    let t100319 = t6272 * t1267;
    let t100321 = t5310 * t92651 * t100319;
    let t100330 = t26960 * t11081 * t29116;
    let t100340 = t1092 * t27763 * t6696 * t1121;
    (t100314, t100319, t100321, t100330, t100340)
}
