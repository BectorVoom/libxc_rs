//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2925/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2925<F: Float>(t4146: F, t1455: F, t5808: F, t1892: F, t9646: F, t9648: F, t1904: F, t47567: F, t14110: F, t47530: F, t1427: F, t1903: F, t22: F, t9647: F) -> (F, F, F, F, F, F) {
    let t47671 = t4146 * t4146;
    let t47672 = F::new(1.0) / t47671;
    let t47730 = t1455 * t5808;
    let t47764 = t9646 * t1892 * t9648;
    let t47772 = t47567 * t1904;
    let t47777 = t47530 * t14110;
    let t47781 = t9647 * t1427 * t1903 * t22;
    (t47672, t47730, t47764, t47772, t47777, t47781)
}
