//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1024/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1024(t112535: f64, t112537: f64, t112542: f64, t115208: f64, t115210: f64, t115212: f64, t115217: f64, t115227: f64, t115229: f64, t115231: f64, t115233: f64, t12823: f64, t2036: f64, t2314: f64, t23909: f64, t24924: f64, t24932: f64, t27888: f64, t32318: f64, t32365: f64, t4034: f64, t7050: f64, t7057: f64, t7266: f64, t8835: f64) -> f64 {
    let t117567 = -2.0_f64 * t12823 * t8835 - t2036 * t24924 - 4.0_f64 * t2314 * t32365 - 2.0_f64 * t23909 * t7266 - 4.0_f64 * t24932 * t7050 - 4.0_f64 * t27888 * t7057 - 4.0_f64 * t32318 * t4034 - t112535 - t112537 - t112542 - t115208 - t115210 - t115212 - t115217 + t115227 - t115229 - t115231 - t115233;
    t117567
}
