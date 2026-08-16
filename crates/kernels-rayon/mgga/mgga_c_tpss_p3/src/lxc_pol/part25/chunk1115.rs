//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1115/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1115(t1027: f64, t15303: f64, t5085: f64, t9267: f64, t4071: f64, t4079: f64, t2868: f64, t5092: f64, t15248: f64, t15251: f64, t15292: f64, t15294: f64, t15296: f64, t15299: f64, t15301: f64) -> (f64, f64, f64, f64, f64) {
    let t15304 = t15303 * t1027;
    let t15306 = t9267 * t5085;
    let t15307 = t15306 * t1027;
    let t15309 = t4071 * t4079;
    let t15311 = t2868 * t5092;
    let t15312 = t15311 * t1027;
    let t15314 = -0.5519e-1_f64 * t15248 + 0.301925e0_f64 * t15251 + 0.258925e1_f64 * t15292 + 0.16504875e0_f64 * t15294 + 0.18396666666666666667e-1_f64 * t15296 - 0.412621875e-1_f64 * t15299 + 0.16504875e0_f64 * t15301 + 0.82524375e-1_f64 * t15304 + 0.19419375e1_f64 * t15307 - 0.258925e1_f64 * t15309 - 0.1294625e1_f64 * t15312;
    (t15304, t15307, t15309, t15312, t15314)
}
