//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 589/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk589(t1777: f64, t4998: f64, t1773: f64, t1786: f64, t25: f64, t698: f64) -> (f64, f64, f64, f64, f64) {
    let t4999 = t4998 * t1777;
    let t5000 = t1773 * t4999;
    let t5002 = t25 * t1786;
    let t5003 = t1773 * t5002;
    let t5005 = 1.0_f64 / t698;
    (t4999, t5000, t5002, t5003, t5005)
}
