//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 691/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk691(t1458: f64, t2075: f64, t2057: f64, t7475: f64, t1492: f64, t2047: f64, t7074: f64, t7076: f64, t7078: f64, t7082: f64, t7494: f64, t7498: f64, t7501: f64, t7504: f64, t7506: f64, t7508: f64) -> (f64, f64, f64, f64) {
    let t7806 = t2075 * t1458;
    let t7809 = t2057 * t7475;
    let t7815 = t1492 * t2047;
    let t7823 = -t7074 - t7494 / 24.0_f64 - t7076 - 0.24223653656484234512e-2_f64 * t7498 - t7078 - 0.40372756094140390853e-3_f64 * t7501 + t7504 / 768.0_f64 - t7506 / 768.0_f64 - t7082 - t7508 / 192.0_f64;
    (t7806, t7809, t7815, t7823)
}
