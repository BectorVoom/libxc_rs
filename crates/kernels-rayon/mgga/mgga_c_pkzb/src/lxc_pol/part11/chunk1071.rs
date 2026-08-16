//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1071/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1071(t1731: f64, t5304: f64, t1730: f64, t1773: f64, t5255: f64, t173: f64, t1764: f64, t614: f64, t1732: f64, t6895: f64, t167: f64, t168: f64, t16942: f64, t180: f64, t66: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17033 = t1731 * t5304;
    let t17034 = t1730 * t17033;
    let t17043 = t1730 * t5255 * t1773;
    let t17051 = t1764 * t173;
    let t17053 = t1730 * t17051 * t614;
    let t17067 = t6895 * t1732;
    let t17088 = 0.28974367305964659283e0_f64 * t167 * t168 / t66 / t16942 * t180;
    (t17033, t17034, t17043, t17051, t17053, t17067, t17088)
}
