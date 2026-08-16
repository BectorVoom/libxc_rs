//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1193/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1193(t28155: f64, t28185: f64, t1378: f64, t1375: f64, t1843: f64, t20029: f64, t20044: f64, t2016: f64, t22646: f64, t26184: f64, t26345: f64, t26477: f64, t26988: f64, t26993: f64, t28051: f64, t28053: f64, t28108: f64, t28111: f64, t28118: f64, t5215: f64, t568: f64, t6461: f64, t6958: f64, t7729: f64, t7750: f64) -> (f64, f64, f64) {
    let t28186 = t28155 + t28185;
    let t28187 = t1378 * t28186;
    let t28190 = 0.76763589786250567036e-1_f64 * t26184 + t26988 + t28051 * t568 + t26993 + 2.0_f64 * t28053 * t568 + t28108 * t568 + 2.0_f64 * t1375 * t28111 - 2.0_f64 * t5215 * t7750 - t22646 + 0.3289868133696452873e-1_f64 * t28118 - 2.0_f64 * t26477 * t1843 - t6958 * t6461 + 4.0_f64 * t5215 * t7729 - t20044 * t2016 - 2.0_f64 * t20029 * t2016 - t1375 * t28187 + 0.82246703342411321824e-2_f64 * t26345;
    (t28186, t28187, t28190)
}
