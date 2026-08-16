//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 804/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk804(t1259: f64, t4516: f64, t1256: f64, t1266: f64, t1657: f64, t3360: f64, t4488: f64, t4490: f64, t4494: f64, t538: f64) -> (f64, f64) {
    let t4517 = t1259 * t4516;
    let t4519 = 2.0_f64 * t1256 * t4494 - t1256 * t4517 - t1266 * t4490 - t1657 * t3360 + t4488 * t538;
    (t4517, t4519)
}
