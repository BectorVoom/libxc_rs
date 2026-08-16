//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1039/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1039(t3255: f64, t5460: f64, t5465: f64, t11633: f64, t1897: f64, t518: f64, t5481: f64, t10338: f64, t1988: f64, t1890: f64, t2323: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16001 = 0.98556445e-3_f64 * t3255 * t5460;
    let t16003 = 0.19711289e-2_f64 * t3255 * t5465;
    let t16025 = t11633 * t1897;
    let t16029 = t518 * t5481;
    let t16038 = t10338 * t1988;
    let t16046 = t2323 * t1890;
    (t16001, t16003, t16025, t16029, t16038, t16046)
}
