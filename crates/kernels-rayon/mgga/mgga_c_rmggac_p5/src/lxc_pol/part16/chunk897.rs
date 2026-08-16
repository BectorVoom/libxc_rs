//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 897/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk897(t14243: f64, t16503: f64, t559: f64, t8425: f64, t14249: f64, t8430: f64, t7414: f64, t9783: f64, t10040: f64, t2186: f64, t2010: f64, t38816: f64, t8465: f64) -> (f64, f64, f64, f64, f64) {
    let t44878 = t16503 * t14243 * t559 * t8425;
    let t44882 = t16503 * t14249 * t559 * t8430;
    let t44886 = t7414 * t9783;
    let t44888 = t2186 * t10040;
    let t44891 = t2010 * t8465 * t38816;
    (t44878, t44882, t44886, t44888, t44891)
}
