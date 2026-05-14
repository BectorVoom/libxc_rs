//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 723/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk723<F: Float>(t1527: F, t1751: F, t1411: F, t378: F, t735: F) -> (F, F, F, F, F) {
    let t4968 = t1751 * t1527;
    let t4969 = 0.32530743900905219526e-1 * t4968;
    let t4970 = t378 * t1411;
    let t4971 = t735 * t4970;
    let t4972 = 0.16265371950452609763e-1 * t4971;
    (t4968, t4969, t4970, t4971, t4972)
}
