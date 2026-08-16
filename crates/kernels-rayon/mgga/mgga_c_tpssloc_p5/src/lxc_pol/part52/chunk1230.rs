//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1230/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1230(t1849: f64, t33084: f64, t33086: f64, t33088: f64, t33092: f64, t33725: f64, t33727: f64, t33731: f64, t33733: f64, t33736: f64, t33740: f64, t652: f64, t7266: f64, t7472: f64, t8329: f64, t8687: f64) -> f64 {
    let t33743 = t1849 * t8687 - 2.0_f64 * t33740 * t652 - 2.0_f64 * t7266 * t7472 + t33084 - 2.0_f64 * t33086 - 2.0_f64 * t33088 - t33092 - t33725 - 2.0_f64 * t33727 - 2.0_f64 * t33731 - 2.0_f64 * t33733 - 2.0_f64 * t33736 - t8329;
    t33743
}
