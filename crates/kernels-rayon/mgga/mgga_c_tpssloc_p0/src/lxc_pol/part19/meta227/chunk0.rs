//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 933/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk933(t10956: f64, t354: f64, t1009: f64, t3020: f64, t1011: f64, t1019: f64, t1040: f64, t3077: f64, t2775: f64, t283: f64, t61: f64, t10305: f64, t248: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10957 = t354 * t10956;
    let t10960 = t3020 * t1009;
    let t10961 = t10960 * t1011;
    let t10962 = t10961 * t1019;
    let t10965 = t3077 * t1040;
    let t10969 = 1.0_f64 / t283 / t2775;
    let t10970 = t61 * t10969;
    let t10972 = t248 * t10970 * t10305;
    (t10957, t10960, t10961, t10962, t10965, t10969, t10970, t10972)
}
