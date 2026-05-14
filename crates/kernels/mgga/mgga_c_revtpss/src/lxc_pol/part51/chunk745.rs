//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 745/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk745<F: Float>(t1544: F, t605: F, t30: F, t4343: F, t1949: F, t4533: F, t7071: F, t689: F, t7774: F, t25411: F, t213: F, t7759: F, t25431: F, t212: F, t780: F, t1032: F, t1568: F) -> (F, F, F, F, F, F, F, F) {
    let t27169 = t605 * t1544;
    let t27173 = t30 * t4343;
    let t27182 = t1949 * t4533;
    let t27183 = t7071 * t27182;
    let t27186 = t7774 * t689;
    let t27187 = t25411 * t27186;
    let t27189 = t213 * t7759;
    let t27192 = t25431 * t27186;
    let t27194 = t212 * t7759;
    let t27195 = t27194 * t780;
    let t27196 = t689 * t27195;
    let t27198 = t1568 * t1032;
    (t27169, t27173, t27183, t27187, t27189, t27192, t27196, t27198)
}
