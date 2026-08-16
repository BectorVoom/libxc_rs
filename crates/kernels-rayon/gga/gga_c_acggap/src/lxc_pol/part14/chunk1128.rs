//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1128/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1128(t1797: f64, t2020: f64, t5586: f64, t570: f64, t2060: f64, t6313: f64, t7815: f64, t6319: f64, t507: f64, t8630: f64, t142: f64, t35364: f64, t6375: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39615 = t2020 * t1797;
    let t39617 = t570 * t5586;
    let t39620 = t2060 * t7815 * t6313;
    let t39623 = t2060 * t7815 * t6319;
    let t39626 = t2060 * t507 * t8630;
    let t39629 = t35364 * t142 * t6375;
    (t39615, t39617, t39620, t39623, t39626, t39629)
}
