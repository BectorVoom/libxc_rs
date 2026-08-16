//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1190/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1190(t12344: f64, t5234: f64, t1831: f64, t40059: f64, t12282: f64, t12290: f64, t12384: f64, t1827: f64, t40123: f64, t1788: f64, t9212: f64, t9214: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t53880 = t5234 * t12344;
    let t53901 = t40059 * t1831;
    let t53945 = t5234 * t12282;
    let t54020 = t5234 * t12290;
    let t54042 = t5234 * t12384;
    let t54151 = t40123 * t1827;
    let t54312 = t9212 * t1788;
    let t54314 = t9214 * t1788;
    (t53880, t53901, t53945, t54020, t54042, t54151, t54312, t54314)
}
