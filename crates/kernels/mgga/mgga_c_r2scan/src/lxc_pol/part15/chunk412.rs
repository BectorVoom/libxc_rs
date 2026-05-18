//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 412/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk412<F: Float>(t1669: F, t612: F, t585: F, t607: F, t159: F, t617: F) -> (F, F, F, F, F) {
    let t1671 = F::new(0.11290853155555555555e-2) * t612 * t1669;
    let t1672 = t607 * t585;
    let t1673 = t159 * t1672;
    let t1674 = t1673 * t617;
    let t1676 = t585 * t585;
    let t1677 = t1676 * t1676;
    let t1678 = t1677 * t585;
    (t1671, t1672, t1673, t1674, t1678)
}
