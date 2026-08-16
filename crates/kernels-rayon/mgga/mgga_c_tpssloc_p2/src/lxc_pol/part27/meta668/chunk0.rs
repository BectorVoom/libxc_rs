//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2355/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2355(t12734: f64, t7461: f64, t2314: f64, t25980: f64, t22574: f64, t56120: f64, t8643: f64, t1845: f64, t3719: f64, t1874: f64, t55962: f64, t19456: f64, t6525: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91591 = 4.0_f64 * t12734 * t7461;
    let t91593 = 4.0_f64 * t2314 * t25980;
    let t91602 = 3.0_f64 * t22574 * t8643 * t56120;
    let t91603 = t1845 * t3719;
    let t91606 = 3.0_f64 * t22574 * t8643 * t91603;
    let t91608 = 2.0_f64 * t55962 * t1874;
    let t91610 = 4.0_f64 * t19456 * t6525;
    (t91591, t91593, t91602, t91606, t91608, t91610)
}
