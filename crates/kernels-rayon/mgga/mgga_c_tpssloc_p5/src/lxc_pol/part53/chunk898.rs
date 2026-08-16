//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 898/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk898(t1266: f64, t1393: f64, t1983: f64, t2036: f64, t2040: f64, t2075: f64, t2314: f64, t23938: f64, t26977: f64, t31057: f64, t31060: f64, t32213: f64, t32220: f64, t32235: f64, t32263: f64, t32278: f64, t4034: f64, t510: f64, t574: f64, t652: f64, t672: f64, t7040: f64, t7042: f64, t7050: f64, t7057: f64, t7061: f64, t7156: f64, t7220: f64, t8607: f64, t8711: f64, t8721: f64, t8780: f64) -> f64 {
    let t32280 = -t31057 - t31060 - 3.0_f64 * t1983 * t32213 - 4.0_f64 * t2314 * t8721 - 4.0_f64 * t4034 * t8721 - 4.0_f64 * t652 * t32220 - 4.0_f64 * t7042 * t7057 - 4.0_f64 * t7042 * t7061 - 2.0_f64 * t8607 * t7220 - 4.0_f64 * t23938 * t2040 - 4.0_f64 * t26977 * t2040 - 4.0_f64 * t7042 * t7050 - 2.0_f64 * t32235 * t672 - 2.0_f64 * t7040 * t2075 - 2.0_f64 * t2036 * t7156 - t32263 * t510 - t8711 * t1266 + t8780 * t1393 + t32278 * t574;
    t32280
}
