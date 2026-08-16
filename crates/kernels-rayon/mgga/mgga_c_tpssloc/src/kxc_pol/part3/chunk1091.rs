//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1091/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1091(t12606: f64, t998: f64, t974: f64, t10868: f64, t1539: f64, t248: f64, t1041: f64, t1009: f64, t4552: f64, t1011: f64, t1019: f64, t1615: f64, t3131: f64) -> (f64, f64, f64, f64, f64) {
    let t14197 = t998 * t12606;
    let t14198 = t974 * t14197;
    let t14202 = t248 * t10868 * t1539;
    let t14203 = t1041 * t14202;
    let t14205 = t4552 * t1009;
    let t14206 = t14205 * t1011;
    let t14207 = t14206 * t1019;
    let t14211 = t1615 * t3131;
    (t14198, t14203, t14205, t14207, t14211)
}
