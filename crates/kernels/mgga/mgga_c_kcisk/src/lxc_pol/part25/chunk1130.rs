//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1130/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1130<F: Float>(t2023: F, t2028: F, t33197: F, t7261: F, t4640: F, t9741: F, t5006: F, t123: F, t9731: F) -> (F, F, F, F, F, F) {
    let t33198 = t2028 * t2023;
    let t33199 = t33197 * t33198;
    let t33200 = t7261 * t33199;
    let t33203 = t9741 * t4640;
    let t33204 = t5006 * t33203;
    let t33207 = t9731 * t123;
    (t33198, t33199, t33200, t33203, t33204, t33207)
}
