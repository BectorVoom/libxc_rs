//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 874/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk874(t34761: f64, t9153: f64, t16502: f64, t8516: f64, t2318: f64, t34976: f64, t7455: f64, t3369: f64, t34975: f64, t559: f64, t35039: f64, t7461: f64) -> (f64, f64, f64, f64) {
    let t39435 = t34761 * t9153;
    let t39437 = t8516 * t16502;
    let t39440 = t39437 * t34976 * t2318 * t7455;
    let t39445 = t34975 * t3369 * t559 * t7455;
    let t39449 = t34975 * t35039 * t2318 * t7461;
    (t39435, t39440, t39445, t39449)
}
