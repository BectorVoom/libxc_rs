//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1142/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1142(t3: f64, t7324: f64, t2127: f64, t3545: f64, t461: f64, t52: f64, t1009: f64, t3502: f64, t3504: f64, t3500: f64, t7337: f64, t483: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24682 = t7324 * t3;
    let t24704 = t2127 * t3545 / 432.0_f64;
    let t24719 = t52 * t461;
    let t24720 = t24719 * t1009;
    let t24721 = t7324 * t24720;
    let t24727 = t3502 * sigma2;
    let t24728 = t24727 * t3504;
    let t24729 = t3500 * t24728;
    let t24732 = t7337 * t3504;
    let t24733 = t3500 * t24732;
    let t24739 = sigma2 * t483;
    (t24682, t24704, t24719, t24720, t24721, t24727, t24728, t24729, t24732, t24733, t24739)
}
