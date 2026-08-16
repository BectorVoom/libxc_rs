//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 464/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk464<F: Float>(t3805: F, t472: F, t300: F, t967: F, t425: F, t1390: F, t143: F, t424: F, t3117: F, t79: F, t435: F, t437: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3806 = t3805 * t472;
    let t3807 = F::cast_from(0.55273148148148148147e-3_f64) * t3806;
    let t3812 = t967 * t300;
    let t3814 = F::cast_from(0.46853067927761790996e-2_f64) * t3812 * t425;
    let t3819 = t143 * t1390;
    let t3830 = t424 * t424;
    let t3831 = F::cast_from(1.0_f64) / t3830;
    let t3841 = t3117 * t79;
    let t3844 = F::cast_from(0.21133333333333333333e-2_f64) * t435 * t3841 * t437;
    (t3806, t3807, t3812, t3814, t3819, t3830, t3831, t3841, t3844)
}
