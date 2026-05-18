//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 488/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk488<F: Float>(t1390: F, t143: F, t3278: F, t425: F, t1056: F, t1354: F, t1364: F, t3283: F, t424: F, t3593: F, t3619: F, t3117: F, t79: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3819 = t143 * t1390;
    let t3820 = t425 * t3278;
    let t3823 = t1354 * t1056;
    let t3824 = t3823 * t1364;
    let t3827 = t425 * t3283;
    let t3830 = t424 * t424;
    let t3831 = F::new(1.0) / t3830;
    let t3832 = t3831 * t3593;
    let t3835 = t1354 * t3619;
    let t3841 = t3117 * t79;
    (t3819, t3820, t3823, t3824, t3827, t3830, t3831, t3832, t3835, t3841)
}
