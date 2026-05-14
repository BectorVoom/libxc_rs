//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 866/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk866<F: Float>(t13665: F, t347: F, t355: F, t13522: F, t1232: F, t4079: F, t346: F, t360: F, t4082: F, t1229: F, t4030: F, t370: F, t4125: F, t1389: F, t381: F, t916: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13666 = 0.73028148148148148147e0 * t13665;
    let t13669 = 1.0 / t347 / t355 / 8.0;
    let t13672 = 0.93011851851851851854e0 * t13522;
    let t13679 = 1.0 / t4079 / t1232;
    let t13680 = t346 * t13679;
    let t13682 = 1.0 / t4082 / t360;
    let t13686 = 0.28842592592592592592e-1 * t13522;
    let t13705 = t1229 * t4030;
    let t13715 = 1.0 / t4125 / t370;
    let t13746 = 0.73586666666666666667e0 * t13665;
    let t13748 = 0.93932222222222222223e0 * t13522;
    let t13776 = 1.0 / t381 / t916 / t1389;
    (t13666, t13669, t13672, t13680, t13682, t13686, t13705, t13715, t13746, t13748, t13776)
}
