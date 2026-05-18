//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 618/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk618<F: Float>(t3202: F, t6620: F, t3200: F, t1646: F, t1773: F, t3211: F) -> (F, F, F) {
    let t6621 = t3202 * t6620;
    let t6622 = t3200 * t6621;
    let t6624 = t1646 * t1773;
    let t6625 = t3211 * t6624;
    (t6621, t6622, t6625)
}
