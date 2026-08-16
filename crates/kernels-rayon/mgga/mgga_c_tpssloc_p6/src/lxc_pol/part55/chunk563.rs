//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 563/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk563(t1530: f64, t2752: f64, t870: f64, t193: f64, t200: f64, t1484: f64, t262: f64, t1877: f64, t202: f64, t2373: f64, t2377: f64, t2522: f64, t4097: f64, t4099: f64, t4100: f64, t4103: f64, t4119: f64, t4198: f64, t4201: f64, t4204: f64, t4207: f64, t4303: f64, t766: f64, t776: f64, t868: f64) -> (f64, f64) {
    let t4307 = t1530 * t2752;
    let t4310 = t1530 * t870;
    let t4314 = t193 * t200;
    let t4315 = t262 * t1484;
    let t4319 = t193 * t202 * t4303 * t870 - t1877 * t4307 * t868 + 3.0_f64 * t193 * t4119 * t766 + 3.0_f64 * t2522 * t4310 * t776 + 6.0_f64 * t4314 * t4315 * t776 + t2373 + t2377 + t4097 + t4099 + t4100 + t4103 + t4198 - t4201 + t4204 + t4207;
    (t4314, t4319)
}
