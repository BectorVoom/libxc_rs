//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1055/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1055(t2310: f64, t7939: f64, t2283: f64, t504: f64, t8619: f64, t8622: f64, t38354: f64, t7473: f64, t7478: f64, t35024: f64, t8451: f64, t36772: f64, t8457: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41882 = t7939 * t2310;
    let t41883 = 0.19863479950205658386e-4_f64 * t41882;
    let t41884 = t7939 * t2283;
    let t41885 = 0.19863479950205658386e-4_f64 * t41884;
    let t41886 = t504 * t8619;
    let t41887 = t41886 * t8622;
    let t41890 = t38354 * t7473;
    let t41891 = t41890 * t7478;
    let t41893 = t8451 * t35024;
    let t41895 = t36772 * t8457;
    (t41883, t41885, t41887, t41891, t41893, t41895)
}
