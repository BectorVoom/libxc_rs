//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1058/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1058<F: Float>(t1078: F, t1976: F, t1982: F, t3140: F, t31966: F, t31970: F, t3057: F, t7165: F, t25669: F, t31999: F, t7150: F, t1045: F, t988: F) -> (F, F, F, F, F) {
    let t120676 = t1982 * t1976 * t3140 * t1078;
    let t120696 = t31966 * t31970;
    let t120702 = t3057 * t7165;
    let t120708 = t7150 * t25669 * t31999;
    let t120709 = t1045 * t988;
    (t120676, t120696, t120702, t120708, t120709)
}
