//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 860/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk860(t13937: f64, t75374: f64, t13940: f64, t75416: f64, t69742: f64, t10570: f64, t14077: f64, t15309: f64, t2046: f64, t2049: f64, t2339: f64, t3167: f64, t39953: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75423 = t13937 * t75374;
    let t75425 = t13940 * t75416;
    let t75440 = 0.59590439850616975158e-4_f64 * t69742;
    let t75443 = t10570 * t14077 * t15309;
    let t75446 = t2046 * t2049 * t2339;
    let t75448 = t39953 * t3167;
    (t75423, t75425, t75440, t75443, t75446, t75448)
}
