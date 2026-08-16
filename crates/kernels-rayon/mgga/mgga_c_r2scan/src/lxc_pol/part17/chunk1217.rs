//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1217/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1217(t3262: f64, t3472: f64, t43802: f64, t12056: f64, t3579: f64, t495: f64, t797: f64, t11199: f64, t3275: f64, t9573: f64, t42978: f64, t42980: f64, t42982: f64, t42985: f64, t42988: f64, t42991: f64, t42994: f64, t42996: f64, t42999: f64, t43002: f64, t43004: f64) -> (f64, f64, f64, f64) {
    let t44161 = 15.0_f64 / 8.0_f64 * t3262 * t3472 * t43802;
    let t44165 = t3579 * t495 * t12056 * t797 / 2.0_f64;
    let t44168 = t3275 * t11199 * t9573 / 2.0_f64;
    let t44181 = -0.46230515946956099003e0_f64 * t42978 + 0.32927245914677557992e0_f64 * t42980 + 0.21951497276451705328e0_f64 * t42982 - 0.17336443480108537126e0_f64 * t42985 - 0.52009330440325611378e0_f64 * t42988 + 0.34672886960217074252e0_f64 * t42991 - 0.10401866088065122276e1_f64 * t42994 - 0.17336443480108537126e0_f64 * t42996 + 0.86682217400542685632e-1_f64 * t42999 + 0.2600466522016280569e0_f64 * t43002 - 0.51220160311720645765e0_f64 * t43004;
    (t44161, t44165, t44168, t44181)
}
