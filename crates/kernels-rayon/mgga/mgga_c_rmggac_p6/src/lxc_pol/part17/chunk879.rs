//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 879/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk879(t38530: f64, t8447: f64, t2318: f64, t34976: f64, t39437: f64, t9145: f64, t3369: f64, t34975: f64, t559: f64, t8455: f64, t16156: f64, t9964: f64) -> (f64, f64, f64, f64) {
    let t44773 = t38530 * t8447;
    let t44777 = t39437 * t34976 * t2318 * t9145;
    let t44781 = t34975 * t3369 * t559 * t8455;
    let t44784 = t16156 * t9964;
    (t44773, t44777, t44781, t44784)
}
