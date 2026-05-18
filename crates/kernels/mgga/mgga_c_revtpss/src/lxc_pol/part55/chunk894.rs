//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 894/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk894<F: Float>(t1544: F, t605: F, t30: F, t4343: F, t1949: F, t4533: F, t7071: F, t689: F, t7774: F) -> (F, F, F, F) {
    let t27169 = t605 * t1544;
    let t27173 = t30 * t4343;
    let t27182 = t1949 * t4533;
    let t27183 = t7071 * t27182;
    let t27186 = t7774 * t689;
    (t27169, t27173, t27183, t27186)
}
