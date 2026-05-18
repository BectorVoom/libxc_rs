//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1113/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1113<F: Float>(t1728: F, t3054: F, t1068: F, t1717: F, t10108: F, t10184: F, t10187: F, t1030: F, t13665: F, t13667: F, t13668: F, t13671: F, t13674: F, t13678: F, t13682: F, t13684: F, t13686: F, t13689: F, t13787: F, t13790: F, t13791: F, t14051: F, t1745: F, t3038: F, t305: F, t3061: F, t313: F, t3158: F, t339: F) -> F {
    let t14053 = t3054 * t1728;
    let t14055 = t1068 * t1717;
    let t14057 = -t13665 - t13667 + F::new(0.46853067927761790996e-2) * t3061 * t13668 + F::new(0.18741227171104716398e-1) * t10108 * t13671 + F::new(0.46853067927761790996e-2) * t1030 * t13674 + F::new(0.18741227171104716398e-1) * t3158 * t13678 - t13682 - t13684 - F::new(0.93706135855523581992e-2) * t10184 + F::new(0.23426533963880895498e-1) * t13686 + t13689 - t3038 * t1745 - F::new(0.46853067927761790996e-2) * t305 * t13787 - F::new(0.18741227171104716398e-1) * t13790 * t13791 - F::new(0.46853067927761790996e-2) * t3158 * t313 - t14051 * t339 - t10187 - F::new(0.93706135855523581992e-2) * t14053 - F::new(0.46853067927761790996e-2) * t14055;
    t14057
}
