//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1731/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1731<F: Float>(t12327: F, t1723: F, t12331: F, t3390: F, t5079: F, t3407: F, t5101: F, t698: F, t16712: F, t1729: F, t2439: F) -> (F, F, F, F, F, F, F, F) {
    let t16851 = t12327 * t1723;
    let t16854 = t12331 * t1723;
    let t16857 = t3390 * t5079;
    let t16862 = t3407 * t5079;
    let t16868 = t698 * t5101;
    let t16869 = F::cast_from(0.10954222222222222222e0_f64) * t16868;
    let t16873 = F::cast_from(0.19931111111111111111e0_f64) * t16712;
    let t16876 = t2439 * t1729;
    (t16851, t16854, t16857, t16862, t16868, t16869, t16873, t16876)
}
