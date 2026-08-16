//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 985/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk985(t1492: f64, t4196: f64, t1487: f64, t13423: f64, t382: f64, t487: f64, t486: f64, t13944: f64, t6369: f64, t6368: f64, t381: f64, t498: f64) -> (f64, f64, f64, f64) {
    let t14524 = t1492 * t4196;
    let t14525 = t1487 * t14524;
    let t14527 = t382 * t13423;
    let t14528 = t487 * t14527;
    let t14529 = t486 * t14528;
    let t14531 = t6369 * t13944;
    let t14532 = t6368 * t14531;
    let t14534 = t381 * t13423;
    let t14535 = t498 * t14534;
    (t14525, t14529, t14532, t14535)
}
