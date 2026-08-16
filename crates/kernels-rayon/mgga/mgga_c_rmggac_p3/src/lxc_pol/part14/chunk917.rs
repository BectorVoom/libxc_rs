//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 917/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk917(t2131: f64, t5321: f64, t1970: f64, t1971: f64, t236: f64, t5601: f64, t38350: f64, t7473: f64, t7478: f64, t1175: f64, t515: f64, t570: f64, t8517: f64) -> (f64, f64, f64, f64) {
    let t39827 = 0.4726e1_f64 * t5321 * t2131;
    let t39830 = t1970 * t1971 * t236 * t5601;
    let t39832 = t38350 * t7473;
    let t39833 = t39832 * t7478;
    let t39838 = t8517 * t1971 * t515 * t570 * t1175;
    (t39827, t39830, t39833, t39838)
}
