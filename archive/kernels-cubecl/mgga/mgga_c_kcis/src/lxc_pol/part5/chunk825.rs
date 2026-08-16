//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 825/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk825<F: Float>(t1662: F, t1727: F, t3274: F, t1103: F, t3279: F, t6272: F, t1104: F, t6276: F, t3288: F, t6320: F, t345: F, t4606: F) -> (F, F, F, F, F, F) {
    let t6578 = t3274 * t1662 * t1727;
    let t6582 = t1103 * t3279 * t6272;
    let t6586 = t1103 * t1104 * t6276;
    let t6589 = t3288 * t6320;
    let t6590 = t345 * t6589;
    let t6593 = t4606 * t1727;
    (t6578, t6582, t6586, t6589, t6590, t6593)
}
