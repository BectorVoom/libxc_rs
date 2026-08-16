//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2493/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2493(t10231: f64, t21122: f64, t973: f64, t13995: f64, t17649: f64, t17681: f64, t21526: f64, t42541: f64, t43382: f64, t50425: f64, t50443: f64, t62891: f64, t62893: f64, t62901: f64, t62903: f64) -> f64 {
    let t70929 = t973 * t10231 * t21122;
    let t70933 = t42541 * t21526 / 768.0_f64 + t13995 * t17681 / 1536.0_f64 + 5.0_f64 / 1296.0_f64 * t50425 - t13995 * t17649 / 768.0_f64 + t62891 / 576.0_f64 - t62893 / 288.0_f64 + t43382 / 10368.0_f64 + t50443 + t70929 / 216.0_f64 + t62901 / 384.0_f64 - t62903 / 768.0_f64;
    t70933
}
