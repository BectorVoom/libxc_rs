//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 718/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk718(t10205: f64, t8500: f64, t8692: f64, t8698: f64, t9037: f64, t9040: f64, t9060: f64, t9062: f64, t9075: f64, t9079: f64, t9091: f64, t117: f64, t5011: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10206 = 0.11974241701863808564e0_f64 * t10205;
    let t10280 = 0.39726959900411316772e-4_f64 * t8500;
    let t10357 = 0.39726959900411316772e-4_f64 * t8692;
    let t10360 = 0.39726959900411316772e-4_f64 * t8698;
    let t10383 = 0.49658699875514145965e-4_f64 * t9037;
    let t10384 = 0.39726959900411316772e-4_f64 * t9040;
    let t10385 = 0.47896966807455234256e0_f64 * t9060;
    let t10386 = 0.3193131120497015617e0_f64 * t9062;
    let t10487 = 0.15965655602485078085e0_f64 * t9075;
    let t10496 = 0.15965655602485078085e0_f64 * t9079;
    let t10504 = 0.39726959900411316772e-4_f64 * t9091;
    let t11905 = t5011 * t117;
    (t10206, t10280, t10357, t10360, t10383, t10384, t10385, t10386, t10487, t10496, t10504, t11905)
}
