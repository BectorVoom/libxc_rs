//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 836/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk836(t1897: f64, t531: f64, t3761: f64, t833: f64, t1317: f64, t509: f64) -> (f64, f64, f64) {
    let t5452 = t1897 * t531;
    let t5454 = t3761 * t5452 * t833;
    let t5457 = t509 * t1317;
    (t5452, t5454, t5457)
}
