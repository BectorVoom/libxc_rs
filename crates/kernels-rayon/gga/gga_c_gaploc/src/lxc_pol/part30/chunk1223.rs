//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1223/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1223(t32356: f64, t7290: f64, t1841: f64, t7289: f64, t2554: f64, t7064: f64, t9006: f64, t10667: f64, t296: f64, t10714: f64, t7137: f64, t10782: f64, t1710: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32357 = t7290 * t32356;
    let t32360 = 0.34180116578409885704e-2_f64 * t1841 * t7289 * t32357;
    let t32362 = t7064 * t9006 * t2554;
    let t32363 = 0.64087718584518535698e-3_f64 * t32362;
    let t32364 = t296 * t10667;
    let t32370 = 0.41016139894091862846e-1_f64 * t7137 * t10714;
    let t32371 = t10782 * t1710;
    (t32357, t32360, t32363, t32364, t32370, t32371)
}
