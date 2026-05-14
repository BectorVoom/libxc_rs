//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1018/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1018<F: Float>(t4054: F, t6051: F, t1237: F, t4046: F, t6059: F, t13536: F, t13650: F, t20302: F, t20402: F, t20406: F, t20409: F, t20412: F, t20415: F, t20417: F, t4037: F, t1248: F, t1249: F, t19136: F) -> (F, F, F, F, F) {
    let t20419 = t4054 * t6051;
    let t20420 = t20419 * t1237;
    let t20422 = t6059 * t4046;
    let t20424 = 0.10064166666666666667e0 * t13536 + 0.11038e0 * t13650 - 0.33114e0 * t20402 + 0.143494e1 * t20406 + 0.22141166666666666666e1 * t20302 + 0.258925e1 * t20409 + 0.19419375e1 * t20412 - 0.412621875e-1 * t20415 - 0.1294625e1 * t20417 + 0.16504875e0 * t20420 + 0.82524375e-1 * t20422;
    let t20426 = t4037 * t6051;
    let t20427 = t20426 * t1237;
    let t20430 = t1248 * t1249 * t19136;
    (t20420, t20422, t20424, t20427, t20430)
}
