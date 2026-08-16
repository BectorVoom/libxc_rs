//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2966/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2966(t10408: f64, t10413: f64, t13977: f64, t13982: f64, t13987: f64, t13991: f64, t14099: f64, t14103: f64, t14508: f64, t14511: f64, t17673: f64, t17693: f64, t3041: f64, t3048: f64, t3070: f64, t3071: f64, t42432: f64, t42561: f64, t4347: f64, t4650: f64, t48548: f64, t48564: f64, t48567: f64, t48570: f64, t48574: f64, t50265: f64, t5677: f64) -> f64 {
    let t61835 = t3070 * t3071 * t4650 * t4347 / 1152.0_f64 + 5.0_f64 / 10368.0_f64 * t48548 + t14508 * t13982 / 768.0_f64 + t48570 * t13987 / 256.0_f64 - t50265 * t13991 / 256.0_f64 - t14511 * t14103 / 1536.0_f64 - t42561 * t17673 / 48.0_f64 + t14508 * t13977 / 384.0_f64 - t14511 * t14099 / 768.0_f64 + t48564 / 576.0_f64 - 5.0_f64 / 648.0_f64 * t3048 * t17693 + 5.0_f64 / 10368.0_f64 * t48567 + t48574 / 2304.0_f64 - t42432 / 20736.0_f64 - 5.0_f64 / 13824.0_f64 * t10413 * t10408 * t5677 * t3041;
    t61835
}
