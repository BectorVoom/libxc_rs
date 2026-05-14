//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 472/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk472<F: Float>(t1364: F, t3823: F, t3283: F, t425: F, t424: F, t3593: F, t1354: F, t3619: F, t3117: F, t79: F, t435: F, t437: F, t3042: F, t41: F, t447: F, t445: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3824 = t3823 * t1364;
    let t3827 = t425 * t3283;
    let t3830 = t424 * t424;
    let t3831 = 1.0 / t3830;
    let t3832 = t3831 * t3593;
    let t3835 = t1354 * t3619;
    let t3841 = t3117 * t79;
    let t3844 = 0.21133333333333333333e-2 * t435 * t3841 * t437;
    let t3845 = t3042 * t41;
    let t3846 = t3845 * t447;
    let t3848 = 0.16804375e-4 * t445 * t3846;
    (t3824, t3827, t3830, t3831, t3832, t3835, t3841, t3844, t3845, t3848)
}
