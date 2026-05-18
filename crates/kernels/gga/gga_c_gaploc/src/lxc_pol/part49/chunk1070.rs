//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1070/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1070<F: Float>(t46849: F, t6508: F, t1358: F, t6507: F, t42529: F, t42533: F, t42537: F, t42540: F, t42544: F, t42547: F, t42551: F, t42570: F, t42573: F, t42575: F) -> (F, F) {
    let t46850 = t6508 * t46849;
    let t46852 = t1358 * t6507 * t46850;
    let t46857 = -F::new(0.63233348079280332442e-2) * t46852 + F::new(0.11856252764865062333e-2) * t42529 - F::new(0.31616674039640166221e-2) * t42533 + t42537 + t42540 + t42544 - t42547 - t42551 - t42570 - t42573 + F::new(0.94850022118920498663e-2) * t42575;
    (t46850, t46857)
}
