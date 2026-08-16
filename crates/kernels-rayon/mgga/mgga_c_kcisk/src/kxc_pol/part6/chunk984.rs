//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 984/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk984(t30273: f64, t6183: f64, t2168: f64, t7744: f64, t3937: f64, t1224: f64, t13538: f64, t30233: f64, t12951: f64, t30153: f64) -> (f64, f64, f64, f64) {
    let t30274 = t6183 * t30273;
    let t30277 = t7744 * t2168;
    let t30278 = t3937 * t30277;
    let t30288 = t1224 * t13538 * t30233;
    let t30290 = t12951 * t30153;
    (t30274, t30278, t30288, t30290)
}
