//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 873/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk873<F: Float>(t146: F, t147: F, t7916: F, t494: F, t938: F, t113: F, t6086: F, t6085: F, t7605: F, t6093: F, t2223: F, t279: F, t6228: F, t6232: F, t6236: F, t6241: F, t6246: F, t7593: F, t7598: F, t7603: F, t7608: F, t7610: F, t7618: F, t7622: F, t7627: F, t7632: F) -> (F, F, F) {
    let t7918 = t146 * t147 * t7916;
    let t7921 = t938 * t494;
    let t7922 = t7921 * t113;
    let t7923 = t6086 * t7922;
    let t7925 = F::cast_from(0.11643651550782197811e-1_f64) * t6085 * t7923;
    let t7926 = t6086 * t7605;
    let t7928 = F::cast_from(0.34930954652346593434e-1_f64) * t6093 * t7926;
    let t7929 = -F::cast_from(0.12805040077930161442e0_f64) * t6228 - F::cast_from(0.23115257973478049502e0_f64) * t6232 + F::cast_from(0.32927245914677557994e0_f64) * t2223 * t7593 - F::cast_from(0.42377972951376424087e0_f64) * t7598 - t7603 - t7608 + t7610 - F::cast_from(0.69345773920434148506e0_f64) * t6236 - F::cast_from(0.42683466926433871472e0_f64) * t6241 + t7618 - t7622 + t7627 + t7632 - F::cast_from(0.16463622957338778997e-1_f64) * t6246 + F::cast_from(0.43341108700271342816e-1_f64) * t7918 * t279 + t7925 + t7928;
    (t7918, t7922, t7929)
}
