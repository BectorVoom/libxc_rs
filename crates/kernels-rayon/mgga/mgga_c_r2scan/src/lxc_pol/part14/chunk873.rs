//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 873/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk873(t146: f64, t147: f64, t7916: f64, t494: f64, t938: f64, t113: f64, t6086: f64, t6085: f64, t7605: f64, t6093: f64, t2223: f64, t279: f64, t6228: f64, t6232: f64, t6236: f64, t6241: f64, t6246: f64, t7593: f64, t7598: f64, t7603: f64, t7608: f64, t7610: f64, t7618: f64, t7622: f64, t7627: f64, t7632: f64) -> (f64, f64, f64) {
    let t7918 = t146 * t147 * t7916;
    let t7921 = t938 * t494;
    let t7922 = t7921 * t113;
    let t7923 = t6086 * t7922;
    let t7925 = 0.11643651550782197811e-1_f64 * t6085 * t7923;
    let t7926 = t6086 * t7605;
    let t7928 = 0.34930954652346593434e-1_f64 * t6093 * t7926;
    let t7929 = -0.12805040077930161442e0_f64 * t6228 - 0.23115257973478049502e0_f64 * t6232 + 0.32927245914677557994e0_f64 * t2223 * t7593 - 0.42377972951376424087e0_f64 * t7598 - t7603 - t7608 + t7610 - 0.69345773920434148506e0_f64 * t6236 - 0.42683466926433871472e0_f64 * t6241 + t7618 - t7622 + t7627 + t7632 - 0.16463622957338778997e-1_f64 * t6246 + 0.43341108700271342816e-1_f64 * t7918 * t279 + t7925 + t7928;
    (t7918, t7922, t7929)
}
