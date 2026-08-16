//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 617/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk617<F: Float>(t1022: F, t6613: F, t1096: F, t1092: F, t1646: F, t1767: F, t3203: F) -> (F, F, F, F) {
    let t6614 = t1022 * t6613;
    let t6615 = t1096 * t6614;
    let t6616 = t1092 * t6615;
    let t6619 = t1646 * t1767;
    let t6620 = t3203 * t6619;
    (t6614, t6615, t6616, t6620)
}
