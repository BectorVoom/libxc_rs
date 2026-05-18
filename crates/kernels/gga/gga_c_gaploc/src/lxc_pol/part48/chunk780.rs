//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 780/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk780<F: Float>(t12568: F, t716: F, t1902: F, t883: F, t12691: F, t2464: F, t825: F, t12704: F, t2684: F, t1645: F, t7696: F, t22980: F, t2615: F, t9438: F) -> (F, F, F, F, F, F) {
    let t40634 = t12568 * t716;
    let t40820 = t883 * t1902;
    let t41060 = t825 * t2464 * t12691;
    let t41071 = t2684 * t2464 * t12704;
    let t41105 = t1645 * t7696;
    let t41231 = t2615 * t9438 * t22980;
    (t40634, t40820, t41060, t41071, t41105, t41231)
}
