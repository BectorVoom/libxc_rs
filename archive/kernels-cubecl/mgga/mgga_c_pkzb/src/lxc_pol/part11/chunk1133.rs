//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1133/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1133<F: Float>(t300: F, t3650: F, t779: F, t2104: F, t5974: F, t9576: F, t9571: F, t5984: F, t9307: F, t17867: F, t3646: F, t9269: F) -> (F, F, F, F, F, F) {
    let t25221 = t300 * t779 * t3650;
    let t25226 = t2104 * t5974 * t9576;
    let t25229 = t2104 * t5974 * t9571;
    let t25231 = t5984 * t9307;
    let t25236 = t2104 * t17867 * t3646;
    let t25239 = t2104 * t5974 * t9269;
    (t25221, t25226, t25229, t25231, t25236, t25239)
}
