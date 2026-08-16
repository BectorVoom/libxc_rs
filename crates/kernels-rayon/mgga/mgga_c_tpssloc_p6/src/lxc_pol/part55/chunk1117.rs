//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1117/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1117(t1458: f64, t8913: f64, t1442: f64, t32666: f64, t32674: f64, t32676: f64, t32679: f64, t32684: f64, t32784: f64, t33084: f64, t33688: f64, t33691: f64, t33693: f64, t33697: f64, t33725: f64, t33727: f64, t33731: f64, t33733: f64, t652: f64, t7266: f64, t7989: f64, t8329: f64) -> (f64, f64) {
    let t34203 = t8913 * t1458;
    let t34210 = -t1442 * t8913 - 2.0_f64 * t34203 * t652 - 4.0_f64 * t7266 * t7989 - t32666 - t32674 - t32676 - t32679 + t32684 + t32784 + t33084 - 4.0_f64 * t33688 - 4.0_f64 * t33691 - 4.0_f64 * t33693 - 4.0_f64 * t33697 - 2.0_f64 * t33725 - 4.0_f64 * t33727 - 4.0_f64 * t33731 - 4.0_f64 * t33733 - t8329;
    (t34203, t34210)
}
