//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 997/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk997(t2310: f64, t7944: f64, t2191: f64, t8597: f64, t7939: f64, t2283: f64, t504: f64, t8619: f64, t8622: f64, t38354: f64, t7473: f64, t7478: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41863 = t7944 * t2310;
    let t41865 = t2191 * t8597;
    let t41882 = t7939 * t2310;
    let t41884 = t7939 * t2283;
    let t41886 = t504 * t8619;
    let t41887 = t41886 * t8622;
    let t41890 = t38354 * t7473;
    let t41891 = t41890 * t7478;
    (t41863, t41865, t41882, t41884, t41887, t41891)
}
