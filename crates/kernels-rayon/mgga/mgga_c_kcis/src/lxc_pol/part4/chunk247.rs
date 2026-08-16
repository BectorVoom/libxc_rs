//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 247/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk247(t921: f64, t924: f64, t261: f64, t257: f64) -> (f64, f64, f64, f64) {
    let t926 = -t921 - 0.17808333333333333333e-1_f64 * t924;
    let t928 = 0.62182e-1_f64 * t926 * t261;
    let t929 = t257 * t257;
    let t930 = 1.0_f64 / t929;
    (t926, t928, t929, t930)
}
