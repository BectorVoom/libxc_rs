//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1199/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1199(t12435: f64, t3308: f64, t3429: f64, t1102: f64, t3314: f64, t10680: f64, t10681: f64, t10683: f64, t3033: f64, t10673: f64, t10674: f64, t10676: f64) -> (f64, f64, f64, f64) {
    let t43829 = t3429 * t3308 * t12435;
    let t43832 = t1102 * t3314 * t12435;
    let t43838 = t10680 * t10681 * t3033 * t10683;
    let t43842 = t10673 * t10674 * t3033 * t10676;
    (t43829, t43832, t43838, t43842)
}
