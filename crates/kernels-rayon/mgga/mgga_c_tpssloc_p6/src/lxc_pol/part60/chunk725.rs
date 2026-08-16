//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 725/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk725(t1244: f64, t24740: f64, t225: f64, t460: f64, t479: f64, t2148: f64, t3427: f64, t2121: f64, t24594: f64, t23598: f64, t50: f64, t131: f64) -> (f64, f64, f64, f64, f64) {
    let t24741 = t1244 * t24740;
    let t24745 = t460 * t225;
    let t24746 = t24745 * t479;
    let t24771 = t3427 * t2148;
    let t24773 = 0.18277045187202515961e-2_f64 * t2121 * t24771;
    let t24776 = t24594 * t225;
    let t24810 = t50 * t23598;
    let t24811 = t24810 * t131;
    (t24741, t24746, t24773, t24776, t24811)
}
