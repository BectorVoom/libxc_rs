//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1479/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1479<F: Float>(t20849: F, t3754: F, t3781: F, t6564: F, t3766: F, t17191: F, t5219: F, t3566: F, t6695: F, t487: F, t69636: F, t17306: F, t1811: F) -> (F, F, F, F, F, F, F) {
    let t72270 = t20849 * t3754;
    let t72326 = t6564 * t3781;
    let t72370 = t6564 * t3766;
    let t72386 = t5219 * t17191;
    let t72767 = t3566 * t6695;
    let t72802 = t69636 * t487;
    let t72874 = t17306 * t1811;
    (t72270, t72326, t72370, t72386, t72767, t72802, t72874)
}
