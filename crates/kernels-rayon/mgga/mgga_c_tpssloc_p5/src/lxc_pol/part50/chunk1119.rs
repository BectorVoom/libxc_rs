//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1119/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1119(t1983: f64, t33157: f64, t1799: f64, t3701: f64, t31084: f64, t113: f64, t1459: f64, t1774: f64, t31224: f64, t33080: f64, t33084: f64, t33086: f64, t33088: f64, t33092: f64, t33096: f64, t33098: f64, t33100: f64, t33101: f64, t33124: f64, t33127: f64, t33131: f64, t33134: f64, t33139: f64, t33155: f64, t510: f64, t574: f64, t8313: f64) -> (f64, f64) {
    let t33158 = t1983 * t33157;
    let t33159 = t3701 * t1799;
    let t33160 = t31084 * t33159;
    let t33162 = 3.0_f64 * t1983 * t33160;
    let t33163 = -t113 * t33080 - 2.0_f64 * t1459 * t31224 - t1774 * t8313 - t33124 * t510 + t33155 * t574 + t33084 - 4.0_f64 * t33086 - 4.0_f64 * t33088 - 2.0_f64 * t33092 - t33096 - t33098 - t33100 - 4.0_f64 * t33101 + 2.0_f64 * t33127 + t33131 + 2.0_f64 * t33134 - t33139 - t33158 - t33162;
    (t33160, t33163)
}
