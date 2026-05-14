//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 511/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk511<F: Float>(t3355: F, t382: F, t1195: F, t1199: F, t2865: F, t41: F, t359: F, t375: F, t1175: F, t1179: F, t3168: F, t355: F, t381: F, t389: F, t143: F, t3038: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3356 = t382 * t3355;
    let t3358 = t1195 * t1199;
    let t3359 = t382 * t3358;
    let t3361 = t2865 * t41;
    let t3362 = t3361 * t359;
    let t3363 = t375 * t3362;
    let t3365 = t1175 * t1179;
    let t3366 = t375 * t3365;
    let t3368 = t3168 * t355;
    let t3369 = t3368 * t381;
    let t3370 = t3369 * t389;
    let t3372 = t3038 * t143;
    (t3356, t3358, t3359, t3361, t3362, t3363, t3365, t3366, t3368, t3369, t3370, t3372)
}
