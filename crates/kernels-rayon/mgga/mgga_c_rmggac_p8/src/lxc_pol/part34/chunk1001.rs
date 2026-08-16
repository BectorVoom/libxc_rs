//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1001/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1001(t75225: f64, t75166: f64, t75169: f64, t75174: f64, t75180: f64, t75184: f64, t77497: f64, t77502: f64, t77503: f64, t77504: f64, t77505: f64, t77506: f64, t77507: f64, t77508: f64, t77509: f64, t77510: f64, t77511: f64) -> f64 {
    let t77512 = 0.2553875993597870364e-4_f64 * t75225;
    let t77513 = 0.10511583655740820313e-5_f64 * t75166 - 0.52557918278704101561e-5_f64 * t75169 - 0.2363e1_f64 * t77497 + 0.29085809927086856923e-4_f64 * t75174 + 0.72714524817717142308e-5_f64 * t75180 - 0.72714524817717142308e-5_f64 * t75184 + t77502 + t77503 + t77504 - t77505 - t77506 + t77507 + t77508 + t77509 - t77510 - t77511 + t77512;
    t77513
}
