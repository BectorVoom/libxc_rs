//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 885/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk885<F: Float>(t2888: F, t9554: F, t2106: F, t3685: F, t2105: F, t2029: F, t3650: F, t2901: F, t302: F, t2923: F, t2976: F, t3645: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9555 = t2888 * t9554;
    let t9558 = t3685 * t2106;
    let t9559 = t2105 * t9558;
    let t9562 = t3650 * t2029;
    let t9563 = t9562 * t2901;
    let t9564 = t302 * t9563;
    let t9567 = t9562 * t2923;
    let t9568 = t302 * t9567;
    let t9571 = t2976 * t3645;
    (t9555, t9558, t9559, t9562, t9563, t9564, t9567, t9568, t9571)
}
