//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1825/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1825(t461: f64, t52: f64, t1009: f64, t7324: f64, t1210: f64, t7330: f64, t3502: f64, t3504: f64, t3500: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24719 = t52 * t461;
    let t24720 = t24719 * t1009;
    let t24721 = t7324 * t24720;
    let t24722 = t1210 * t7330;
    let t24723 = t24721 * t24722;
    let t24727 = t3502 * sigma2;
    let t24728 = t24727 * t3504;
    let t24729 = t3500 * t24728;
    (t24719, t24720, t24721, t24722, t24723, t24727, t24728, t24729)
}
