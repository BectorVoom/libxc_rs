//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 732/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk732<F: Float>(t13522: F, t370: F, t4125: F, t13665: F, t1389: F, t381: F, t916: F, t1318: F, t398: F, t13614: F, t397: F, t403: F) -> (F, F, F, F, F, F, F) {
    let t13686 = F::cast_from(0.28842592592592592592e-1_f64) * t13522;
    let t13715 = F::cast_from(1.0_f64) / t4125 / t370;
    let t13746 = F::cast_from(0.73586666666666666667e0_f64) * t13665;
    let t13748 = F::cast_from(0.93932222222222222223e0_f64) * t13522;
    let t13776 = F::cast_from(1.0_f64) / t381 / t916 / t1389;
    let t13829 = t1318 * t1318;
    let t13830 = F::cast_from(1.0_f64) / t13829;
    let t13831 = t398 * t13830;
    let t13871 = t397 * t13614 * t403;
    (t13686, t13715, t13746, t13748, t13776, t13831, t13871)
}
