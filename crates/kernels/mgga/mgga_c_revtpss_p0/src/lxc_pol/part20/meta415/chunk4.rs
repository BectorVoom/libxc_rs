//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1539/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1539<F: Float>(t11870: F, t11922: F, t3115: F, t11631: F, t3133: F, t1086: F, t11223: F, t3090: F, t11866: F, t11923: F, t11200: F, t1043: F, t11202: F) -> (F, F, F, F, F, F) {
    let t43277 = t3115 * t11922 * t11870;
    let t43279 = t11631 * t3133;
    let t43285 = t11223 * t1086 * t3090;
    let t43288 = t11866 * t11923;
    let t43291 = t11200 * t1086 * t3090;
    let t43292 = t11202 * t1043;
    (t43277, t43279, t43285, t43288, t43291, t43292)
}
