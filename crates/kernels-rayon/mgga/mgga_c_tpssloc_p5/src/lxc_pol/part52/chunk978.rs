//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 978/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk978(t483: f64, t3068: f64, t1244: f64, t2132: f64, t24683: f64, t225: f64, t460: f64, t479: f64, t3523: f64, t7345: f64, t3572: f64, t7339: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24739 = sigma2 * t483;
    let t24740 = t24739 * t3068;
    let t24741 = t1244 * t24740;
    let t24744 = t2132 * t24683;
    let t24745 = t460 * t225;
    let t24746 = t24745 * t479;
    let t24747 = t24744 * t24746;
    let t24752 = t7345 * t3523;
    let t24754 = t7339 * t3572;
    (t24741, t24745, t24746, t24747, t24752, t24754)
}
