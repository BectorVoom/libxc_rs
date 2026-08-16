//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 847/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk847(t41817: f64, t388: f64, t535: f64, t7933: f64, t7934: f64, t7244: f64, t8422: f64, t2310: f64, t7939: f64, t2283: f64, t38354: f64, t7473: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41818 = 0.72042316457491791906e-3_f64 * t41817;
    let t41821 = t7933 * t7934 * t388 * t535;
    let t41822 = 0.72042316457491791906e-3_f64 * t41821;
    let t41828 = t7244 * t8422;
    let t41829 = 0.19863479950205658386e-4_f64 * t41828;
    let t41882 = t7939 * t2310;
    let t41883 = 0.19863479950205658386e-4_f64 * t41882;
    let t41884 = t7939 * t2283;
    let t41885 = 0.19863479950205658386e-4_f64 * t41884;
    let t41890 = t38354 * t7473;
    (t41818, t41822, t41829, t41883, t41885, t41890)
}
