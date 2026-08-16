//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 560/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk560(t2844: f64, t291: f64, t2630: f64, t2888: f64, t1000: f64, t993: f64) -> (f64, f64, f64) {
    let t2889 = t291 * t2844;
    let t2890 = t2889 * t2630;
    let t2891 = t2888 * t2890;
    let t2894 = t993 * t1000;
    (t2890, t2891, t2894)
}
