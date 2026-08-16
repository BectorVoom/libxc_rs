//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 822/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk822(t34884: f64, t8668: f64, t8831: f64, t8836: f64, t8843: f64, t2320: f64, t35151: f64, t2604: f64, t8997: f64, t1679: f64, t7900: f64, t36662: f64, t8417: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40558 = t34884 * t8668;
    let t40560 = t34884 * t8831;
    let t40562 = t34884 * t8836;
    let t40564 = t34884 * t8843;
    let t40566 = t35151 * t2320;
    let t40578 = t2604 * t8997;
    let t40623 = t1679 * t7900;
    let t40654 = t36662 * t8417;
    (t40558, t40560, t40562, t40564, t40566, t40578, t40623, t40654)
}
