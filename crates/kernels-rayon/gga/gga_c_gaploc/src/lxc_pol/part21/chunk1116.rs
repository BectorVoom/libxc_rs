//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1116/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1116(t21446: f64, t739: f64, t3248: f64, t7211: f64, t2549: f64, t9625: f64, t1949: f64, t3240: f64, t731: f64, t9630: f64, t21483: f64, t2562: f64, t883: f64, t943: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29194 = t739 * t21446;
    let t29210 = 0.64087718584518535698e-3_f64 * t7211 * t3248;
    let t29212 = 0.1281754371690370714e-2_f64 * t2549 * t9625;
    let t29224 = 0.17090058289204942853e-2_f64 * t1949 * t3240;
    let t29226 = 0.17090058289204942853e-2_f64 * t731 * t9630;
    let t29230 = 0.64087718584518535698e-3_f64 * t943 * t2562 * t883 * t21483;
    (t29194, t29210, t29212, t29224, t29226, t29230)
}
