//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 934/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk934(t1997: f64, t45522: f64, t10084: f64, t16043: f64, t511: f64, t6304: f64, t650: f64, t1525: f64, t1971: f64, t515: f64, t570: f64, t7230: f64) -> (f64, f64, f64, f64) {
    let t45523 = t45522 * t1997;
    let t45525 = t16043 * t10084;
    let t45530 = t6304 * t511;
    let t45531 = t45530 * t650;
    let t45536 = t7230 * t1971 * t515 * t570 * t1525;
    (t45523, t45525, t45531, t45536)
}
