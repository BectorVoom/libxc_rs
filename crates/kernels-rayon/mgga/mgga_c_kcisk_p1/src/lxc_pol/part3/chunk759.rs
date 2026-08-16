//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 759/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk759(t1906: f64, t724: f64, t1957: f64, t5219: f64, t5339: f64, t5218: f64, t10534: f64, t5290: f64, t5289: f64, t10381: f64, t7315: f64, t10431: f64) -> (f64, f64, f64, f64, f64) {
    let t11699 = t1906 * t1906;
    let t11700 = 1.0_f64 / t11699;
    let t11701 = t724 * t11700;
    let t11702 = t5219 * t1957;
    let t11704 = 6.0_f64 * t11701 * t11702;
    let t11705 = t1957 * t5339;
    let t11707 = 6.0_f64 * t5218 * t11705;
    let t11708 = t5290 * t10534;
    let t11709 = t5289 * t11708;
    let t11711 = t5290 * t10381;
    let t11712 = t7315 * t11711;
    let t11714 = t5290 * t10431;
    (t11704, t11707, t11709, t11712, t11714)
}
