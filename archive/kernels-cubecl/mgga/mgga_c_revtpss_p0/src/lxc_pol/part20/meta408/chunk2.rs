//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1512/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1512<F: Float>(t1007: F, t11738: F, t3080: F, t3083: F, t1043: F, t11173: F, t11858: F, t16048: F, t11859: F, t11861: F, t11922: F, t11927: F, t11929: F) -> (F, F, F, F, F, F) {
    let t42754 = t11738 * t1007;
    let t42756 = t3083 * t3080;
    let t42760 = t11173 * t1043;
    let t42765 = t11858 * t16048;
    let t42769 = t11859 * t11922 * t11861;
    let t42772 = t11927 * t11922 * t11929;
    (t42754, t42756, t42760, t42765, t42769, t42772)
}
