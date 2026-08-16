//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 353/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk353(t1167: f64, t179: f64, t932: f64, t1220: f64, t1224: f64, t1230: f64, t1238: f64, t385: f64, t388: f64, t404: f64, t407: f64, t906: f64, t918: f64, t929: f64) -> f64 {
    let t1242 = t179 * t932 * t1167;
    let t1245 = -t1220 * t388 / 36.0_f64 + t906 - t385 * t1224 / 96.0_f64 + 0.21437009059034868486e-3_f64 * t918 * t1230 - 0.11433071498151929859e-2_f64 * t1238 * t407 + t929 - 0.42874018118069736972e-3_f64 * t404 * t1242;
    t1245
}
