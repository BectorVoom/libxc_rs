//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 902/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk902(t1557: f64, t5726: f64, t2792: f64, t1556: f64, t17520: f64, t2842: f64, t1569: f64, t5758: f64, t10636: f64, t13598: f64, t17149: f64, t17165: f64, t17175: f64, t21124: f64, t21128: f64, t21147: f64, t21150: f64, t21153: f64, t21156: f64) -> (f64, f64, f64, f64) {
    let t21315 = t1557 * t5726;
    let t21317 = 6.0_f64 * t2792 * t21315;
    let t21318 = t17520 * t1556;
    let t21320 = 0.48245938496077605201e2_f64 * t2842 * t21318;
    let t21321 = t1569 * t5758;
    let t21334 = -t10636 - 0.23744444444444444444e-1_f64 * t13598 + 0.11872222222222222222e-1_f64 * t17149 - 0.35616666666666666666e-1_f64 * t17165 + 0.17808333333333333333e-1_f64 * t17175 - 0.19787037037037037037e-1_f64 * t21147 + 0.71233333333333333332e-1_f64 * t21150 - 0.35616666666666666666e-1_f64 * t21124 - 0.10685e0_f64 * t21153 + 0.10685e0_f64 * t21128 - 0.17808333333333333333e-1_f64 * t21156;
    (t21317, t21320, t21321, t21334)
}
