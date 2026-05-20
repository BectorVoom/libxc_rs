//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2113/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2113<F: Float>(t15154: F, t2908: F, t141: F, t15158: F, t930: F, t4625: F, t698: F, t4622: F) -> (F, F, F, F, F, F, F) {
    let t15162 = t2908 * t15154;
    let t15163 = t141 * t15162;
    let t15165 = t930 * t15158;
    let t15166 = t141 * t15165;
    let t15168 = t698 * t4625;
    let t15169 = F::new(0.22076e0) * t15168;
    let t15170 = t698 * t4622;
    (t15162, t15163, t15165, t15166, t15168, t15169, t15170)
}
