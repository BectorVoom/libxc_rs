//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 852/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk852(t12321: f64, t41: f64, t4291: f64, t5747: f64, t2033: f64, t4121: f64, t492: f64, t6015: f64, t1466: f64, t5997: f64, t11825: f64, t12534: f64, t251: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17382 = t41 * t12321;
    let t17391 = t5747 * t4291;
    let t17396 = t2033 * t4121;
    let t17412 = t6015 * t492;
    let t17449 = t5997 * t1466;
    let t17450 = t17449 * sigma2;
    let t17463 = t11825 * t4291;
    let t17470 = t251 * t12534;
    (t17382, t17391, t17396, t17412, t17449, t17450, t17463, t17470)
}
