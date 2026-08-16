//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1022/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1022(t14447: f64, t4567: f64, t991: f64, t2469: f64, t992: f64, t4952: f64, t291: f64, t9897: f64, t2887: f64, t736: f64, t1245: f64, t4967: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14448 = t14447 * t4567;
    let t14450 = t991 * t14448 / 324.0_f64;
    let t14453 = t2469 * t992;
    let t14454 = t14453 * t4952;
    let t14455 = t991 * t14454;
    let t14492 = t9897 * t291;
    let t14496 = t736 * t2887;
    let t14497 = t14496 * t291;
    let t14516 = t1245 * t4967;
    (t14450, t14455, t14492, t14496, t14497, t14516)
}
