//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 893/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk893(t31043: f64, t8808: f64, t649: f64, t8717: f64, t113: f64, t1266: f64, t1983: f64, t2096: f64, t31055: f64, t31304: f64, t32108: f64, t32111: f64, t32187: f64, t32189: f64, t32194: f64, t32197: f64, t32200: f64, t510: f64, t650: f64, t652: f64, t6876: f64, t7057: f64, t7171: f64, t7218: f64, t8329: f64, t8607: f64, t8718: f64, t8774: f64, t8805: f64, t8809: f64, t9003: f64) -> (f64, f64, f64) {
    let t32203 = t8808 * t31043;
    let t32206 = t649 * t8717;
    let t32211 = -t113 * t32108 - 2.0_f64 * t1266 * t8718 + 3.0_f64 * t1983 * t32111 + t1983 * t32187 - t1983 * t32189 - 2.0_f64 * t1983 * t32194 + 2.0_f64 * t1983 * t32203 + 2.0_f64 * t2096 * t31304 - 2.0_f64 * t32197 * t652 - 4.0_f64 * t32200 * t652 - 2.0_f64 * t32206 * t510 - t650 * t8774 + t6876 * t8805 - t6876 * t8809 - 4.0_f64 * t7057 * t9003 + 6.0_f64 * t7171 * t8607 + 2.0_f64 * t7218 * t8607 - t31055 - t8329;
    (t32203, t32206, t32211)
}
