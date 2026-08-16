//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 799/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk799(t1009: f64, t4977: f64, t1697: f64, t978: f64, t1121: f64, t1773: f64) -> (f64, f64, f64) {
    let t4978 = t4977 * t1009;
    let t4981 = t1697 * t978;
    let t4984 = t1773 * t1121;
    (t4978, t4981, t4984)
}
