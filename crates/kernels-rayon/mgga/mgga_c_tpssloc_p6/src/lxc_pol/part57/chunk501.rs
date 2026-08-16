//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 501/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk501(t1241: f64, t6267: f64, t1238: f64, t1761: f64, t4945: f64, t498: f64, t5055: f64, t6151: f64, t6153: f64, t6239: f64, t6244: f64, t1763: f64) -> (f64, f64) {
    let t6268 = t1241 * t6267;
    let t6270 = 2.0_f64 * t1238 * t6244 - t1238 * t6268 - 2.0_f64 * t1761 * t4945 - 2.0_f64 * t1761 * t5055 + t498 * t6151 + 2.0_f64 * t498 * t6153 + t498 * t6239;
    let t6274 = t1763 * t1763;
    (t6270, t6274)
}
