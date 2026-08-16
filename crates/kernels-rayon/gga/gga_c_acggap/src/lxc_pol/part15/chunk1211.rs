//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1211/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1211(t34043: f64, t34054: f64, t34056: f64, t34058: f64, t34059: f64, t36918: f64, t38929: f64, t38934: f64, t38937: f64, t38939: f64, t38942: f64, t38946: f64, t38950: f64, t38954: f64, t38958: f64, t38960: f64, t38964: f64, t38968: f64) -> f64 {
    let t41421 = 0.76220476654346199062e-2_f64 * t34043 - 0.7145669686344956162e-3_f64 * t38929 - t36918 - 0.52832795046534975476e-1_f64 * t34054 - 0.42874018118069736972e-3_f64 * t38934 - 0.28582678745379824648e-2_f64 * t34056 - t34058 + 0.68598428988911579155e-1_f64 * t38937 + 0.37737710747524982483e-2_f64 * t38939 + t38942 / 16.0_f64 + 0.25158473831683321655e-2_f64 * t34059 + 0.42874018118069736972e-2_f64 * t38946 + 0.42874018118069736972e-2_f64 * t38950 + 0.42874018118069736972e-2_f64 * t38954 + 0.28582678745379824648e-2_f64 * t38958 - 0.94344276868812456206e-2_f64 * t38960 - 0.94344276868812456206e-2_f64 * t38964 - 0.94344276868812456206e-2_f64 * t38968;
    t41421
}
