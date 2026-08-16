//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1212/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1212(t2776: f64, t6785: f64, t6784: f64, t1003: f64, t1058: f64, t1953: f64, t23346: f64, t23601: f64, t23666: f64, t23670: f64, t23674: f64, t23680: f64, t23687: f64, t23693: f64, t23698: f64, t23701: f64, t23705: f64, t23707: f64, t23712: f64, t3076: f64, t3186: f64, t353: f64, t6680: f64, t6687: f64, t6787: f64, t6790: f64, t6797: f64, t6802: f64, t6806: f64, t6813: f64) -> (f64, f64, f64) {
    let t23714 = t6785 * t2776;
    let t23715 = t6784 * t23714;
    let t23720 = 0.54831135561607547884e-2_f64 * t23666 - 0.43864908449286038306e-1_f64 * t23670 * t6802 + 0.82246703342411321825e-2_f64 * t6797 * t23674 + 0.16449340668482264365e-1_f64 * t23601 * t23680 - 0.43864908449286038306e-1_f64 * t6680 * t6806 + 0.54831135561607547884e-2_f64 * t6687 * t23687 - 0.14621636149762012769e-1_f64 * t23346 * t6787 + 0.27415567780803773942e-2_f64 * t6687 * t23693 + 0.36554090374405031923e-2_f64 * t6687 * t23698 + 2.0_f64 * t3186 * t23701 + t1058 * t23705 + t353 * t23707 + t3076 * t1953 + 2.0_f64 * t1003 * t6813 + 0.18277045187202515961e-2_f64 * t23712 - 0.54831135561607547884e-2_f64 * t6687 * t23715 + 0.43864908449286038306e-1_f64 * t23346 * t6790;
    (t23714, t23715, t23720)
}
