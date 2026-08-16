//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 926/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk926(t2147: f64, t39953: f64, t5055: f64, t7524: f64, t36895: f64, t8571: f64, t35535: f64, t36450: f64, t8443: f64, t36734: f64, t1475: f64, t1970: f64, t1971: f64, t875: f64, t876: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39954 = t39953 * t2147;
    let t39956 = t5055 * t7524;
    let t39964 = t8571 * t36895;
    let t39966 = t8571 * t35535;
    let t39968 = t36450 * t8443;
    let t39970 = t36734 * t8443;
    let t39971 = 0.19863479950205658386e-4_f64 * t39970;
    let t39975 = t1970 * t1971 * t875 * t1475 * t876;
    (t39954, t39956, t39964, t39966, t39968, t39971, t39975)
}
