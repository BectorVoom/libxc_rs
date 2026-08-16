//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 880/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk880(t39570: f64, t8906: f64, t623: f64, t8619: f64, t8622: f64, t16504: f64, t34975: f64, t552: f64, t8455: f64, t2344: f64, t40193: f64, t1368: f64, t16503: f64, t3369: f64, t8435: f64) -> (f64, f64, f64, f64, f64) {
    let t44786 = t39570 * t8906;
    let t44788 = t623 * t8619;
    let t44789 = t44788 * t8622;
    let t44793 = t34975 * t16504 * t552 * t8455;
    let t44795 = t40193 * t2344;
    let t44799 = t16503 * t3369 * t1368 * t8435;
    (t44786, t44789, t44793, t44795, t44799)
}
