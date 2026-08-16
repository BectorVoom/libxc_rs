//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2628/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2628(t12339: f64, t5310: f64, t16150: f64, t3866: f64, t16155: f64, t1827: f64, t40123: f64, t1824: f64, t3850: f64, t16060: f64, t3802: f64, t1799: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54133 = t12339 * t5310;
    let t54135 = t3866 * t16150;
    let t54138 = t3866 * t16155;
    let t54151 = t40123 * t1827;
    let t54153 = t1824 * t3850;
    let t54162 = t16060 * t3802;
    let t54165 = t1799 * t3850;
    (t54133, t54135, t54138, t54151, t54153, t54162, t54165)
}
