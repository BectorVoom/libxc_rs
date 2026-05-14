//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 994/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk994<F: Float>(t19849: F, t5992: F, t1411: F, t5886: F, t5997: F, t1284: F, t2231: F, t6006: F, t3796: F, t19067: F, t1163: F, t8240: F, t14265: F, t3482: F, t5975: F, t3739: F, t7839: F) -> (F, F, F, F, F, F, F, F) {
    let t26766 = t19849 * t5992;
    let t26767 = t1411 * t26766;
    let t26769 = t5886 * t5997;
    let t26770 = t1411 * t26769;
    let t26773 = t1284 * t2231;
    let t26774 = t26773 * t6006;
    let t26775 = t3796 * t26774;
    let t26776 = t19067 * t26775;
    let t26778 = t8240 * t1163;
    let t26779 = t14265 * t26778;
    let t26780 = t3482 * t26779;
    let t26782 = t5886 * t5975;
    let t26783 = t1411 * t26782;
    let t26785 = t3739 * t7839;
    (t26767, t26770, t26774, t26776, t26778, t26780, t26783, t26785)
}
