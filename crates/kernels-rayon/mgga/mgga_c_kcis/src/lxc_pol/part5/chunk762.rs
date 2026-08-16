//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 762/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk762(t509: f64, t5869: f64, t552: f64, t557: f64, t303: f64, t1497: f64, t2011: f64) -> (f64, f64, f64, f64) {
    let t5870 = t509 * t5869;
    let t5871 = t5870 * t552;
    let t5872 = t5871 * t557;
    let t5873 = t303 * t5872;
    let t5875 = t2011 * t1497;
    (t5871, t5872, t5873, t5875)
}
