//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 838/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk838(t2032: f64, t23975: f64, t26055: f64, t26063: f64, t26067: f64, t26070: f64, t26073: f64, t26076: f64, t26090: f64, t26911: f64, t26920: f64, t26936: f64, t6492: f64, t6495: f64, t7026: f64, t7035: f64, t7432: f64, t7435: f64, t7782: f64) -> f64 {
    let t26938 = -5.0_f64 / 3.0_f64 * t26911 * t6492 - 2.0_f64 / 3.0_f64 * t26055 * t2032 - 5.0_f64 / 3.0_f64 * t23975 * t7432 - 5.0_f64 / 3.0_f64 * t7026 * t26063 + 40.0_f64 / 9.0_f64 * t26920 - 5.0_f64 / 3.0_f64 * t7026 * t26067 - 2.0_f64 / 3.0_f64 * t26070 * t2032 - 2.0_f64 / 3.0_f64 * t26073 * t2032 - 2.0_f64 / 3.0_f64 * t26076 * t2032 - 2.0_f64 / 3.0_f64 * t7435 * t7035 - 5.0_f64 / 3.0_f64 * t7026 * t26090 - 2.0_f64 / 3.0_f64 * t6495 * t7782 + 16.0_f64 / 9.0_f64 * t26936;
    t26938
}
