//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1340/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1340<F: Float>(t1134: F, t16862: F, t3399: F, t5087: F, t5101: F, t698: F, t1145: F, t16746: F, t141: F, t16712: F, t1729: F, t2439: F) -> (F, F, F, F, F, F, F) {
    let t16863 = t16862 * t1134;
    let t16865 = t5087 * t3399;
    let t16868 = t698 * t5101;
    let t16869 = F::new(0.10954222222222222222e0) * t16868;
    let t16870 = t1145 * t16746;
    let t16871 = t141 * t16870;
    let t16873 = F::new(0.19931111111111111111e0) * t16712;
    let t16876 = t2439 * t1729;
    (t16863, t16865, t16868, t16869, t16871, t16873, t16876)
}
