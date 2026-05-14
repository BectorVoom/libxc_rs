//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1250/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1250<F: Float>(t16851: F, t3391: F, t12331: F, t1723: F, t3390: F, t5079: F, t1134: F, t3399: F, t5071: F, t3407: F, t5087: F, t5101: F, t698: F, t1145: F, t16746: F, t141: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16852 = t16851 * t3391;
    let t16854 = t12331 * t1723;
    let t16855 = t16854 * t3391;
    let t16857 = t3390 * t5079;
    let t16858 = t16857 * t1134;
    let t16860 = t5071 * t3399;
    let t16862 = t3407 * t5079;
    let t16863 = t16862 * t1134;
    let t16865 = t5087 * t3399;
    let t16868 = t698 * t5101;
    let t16869 = 0.10954222222222222222e0 * t16868;
    let t16870 = t1145 * t16746;
    let t16871 = t141 * t16870;
    (t16852, t16855, t16858, t16860, t16863, t16865, t16868, t16869, t16871)
}
