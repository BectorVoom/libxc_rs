//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 900/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk900(t24727: f64, t3504: f64, t3500: f64, t7337: f64, t1202: f64, t7344: f64, t483: f64, t3068: f64, t1244: f64, t2132: f64, t24683: f64, t225: f64, t460: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24728 = t24727 * t3504;
    let t24729 = t3500 * t24728;
    let t24732 = t7337 * t3504;
    let t24733 = t3500 * t24732;
    let t24736 = t1202 * t7344;
    let t24739 = sigma2 * t483;
    let t24740 = t24739 * t3068;
    let t24741 = t1244 * t24740;
    let t24744 = t2132 * t24683;
    let t24745 = t460 * t225;
    (t24729, t24733, t24736, t24741, t24744, t24745)
}
