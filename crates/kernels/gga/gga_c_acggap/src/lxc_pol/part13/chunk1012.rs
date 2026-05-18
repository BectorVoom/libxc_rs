//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1012/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1012<F: Float>(t33953: F, t5150: F, t13364: F, t33952: F, t154: F, t506: F, t7322: F, t7326: F, t7315: F, t8589: F, t2046: F, t336: F, t4099: F, t579: F) -> (F, F, F, F, F) {
    let t33954 = t33953 * t5150;
    let t33956 = t33952 * t13364 * t33954;
    let t33960 = t7322 * t154 * t506 * t7326;
    let t33962 = t7315 * t8589;
    let t33963 = F::new(11.0) / F::new(192.0) * t33962;
    let t33966 = t2046 * t336 * t579 * t4099;
    (t33954, t33956, t33960, t33963, t33966)
}
