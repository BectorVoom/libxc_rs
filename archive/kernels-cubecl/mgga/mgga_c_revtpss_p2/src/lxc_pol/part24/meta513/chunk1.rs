//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1531/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1531<F: Float>(t23820: F, t73: F, t1063: F, t23485: F, t247: F, t3109: F, t11922: F, t23993: F, t3115: F, t23935: F, t4899: F, t15932: F, t19826: F) -> (F, F, F, F, F) {
    let t79159 = t23820 * t73;
    let t79219 = t1063 * t247 * t3109 * t23485;
    let t79233 = t3115 * t11922 * t23993;
    let t79253 = t4899 * t11922 * t23935;
    let t79290 = t15932 * t19826;
    (t79159, t79219, t79233, t79253, t79290)
}
