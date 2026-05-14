//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 825/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk825<F: Float>(t13522: F, t1229: F, t4030: F, t370: F, t4125: F, t13665: F, t1389: F, t381: F, t916: F, t1323: F, t164: F, t1309: F, t1318: F, t1294: F, t3981: F, t1301: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13686 = 0.28842592592592592592e-1 * t13522;
    let t13705 = t1229 * t4030;
    let t13715 = 1.0 / t4125 / t370;
    let t13746 = 0.73586666666666666667e0 * t13665;
    let t13748 = 0.93932222222222222223e0 * t13522;
    let t13776 = 1.0 / t381 / t916 / t1389;
    let t13804 = t164 * t1323;
    let t13805 = t1309 * t13804;
    let t13829 = t1318 * t1318;
    let t13830 = 1.0 / t13829;
    let t13861 = t1294 * t3981;
    let t13868 = t1301 * t3981;
    (t13686, t13705, t13715, t13746, t13748, t13776, t13805, t13830, t13861, t13868)
}
