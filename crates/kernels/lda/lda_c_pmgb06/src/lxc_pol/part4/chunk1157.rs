//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1157/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1157<F: Float>(t15274: F, t5068: F, t529: F, t6559: F, t1586: F, t6560: F, t16825: F, t5077: F, t5084: F, t2386: F, t337: F, t12529: F, t12530: F, t12535: F, t13300: F, t17070: F, t3247: F, t5065: F) -> (F, F, F, F, F, F) {
    let t17448 = 8.0 / 45.0 * t5068 * t6559 * t15274 * t529;
    let t17452 = 4.0 / 45.0 * t5068 * t6559 * t6560 * t1586;
    let t17455 = 4.0 / 15.0 * t5077 * t5084 * t16825;
    let t17457 = t2386 * t337 * t529;
    let t17460 = 16.0 / 81.0 * t12529 * t12530 * t17457;
    let t17465 = 64.0 / 81.0 * t5065 * t12535 * t3247 * t13300 * t17070;
    (t17448, t17452, t17455, t17457, t17460, t17465)
}
