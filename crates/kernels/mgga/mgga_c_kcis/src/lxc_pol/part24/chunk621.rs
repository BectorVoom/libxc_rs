//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 621/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk621<F: Float>(t3330: F, t6638: F, t143: F, t6432: F, t3399: F, t3400: F, t6272: F, t1154: F, t1646: F, t5153: F, t3410: F, t1155: F, t6276: F) -> (F, F, F, F, F, F) {
    let t6640 = F::cast_from(2.0_f64) * t3330 * t6638;
    let t6641 = t6432 * t143;
    let t6661 = t3399 * t3400 * t6272;
    let t6665 = t1154 * t5153 * t1646;
    let t6669 = t1154 * t3410 * t6272;
    let t6673 = t1154 * t1155 * t6276;
    (t6640, t6641, t6661, t6665, t6669, t6673)
}
