//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1390/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1390(t11039: f64, t3265: f64, t1096: f64, t31777: f64, t34318: f64, t34321: f64, t34323: f64, t34325: f64, t34328: f64, t34344: f64, t34346: f64, t34351: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36892 = t3265 * t11039;
    let t36894 = t31777 * t1096;
    let t36896 = 0.21135226489492151266e-6_f64 * t34318;
    let t36897 = 0.42206481990611010728e-7_f64 * t34321;
    let t36898 = 0.21103240995305505364e-7_f64 * t34323;
    let t36899 = 0.13506074236995523433e-5_f64 * t34325;
    let t36900 = 0.63350674672043801542e-5_f64 * t34328;
    let t36906 = 0.11594181388521408695e-4_f64 * t34344;
    let t36907 = 0.43440462632258606772e-4_f64 * t34346;
    let t36908 = 0.50680539737635041234e-3_f64 * t34351;
    (t36892, t36894, t36896, t36897, t36898, t36899, t36900, t36906, t36907, t36908)
}
