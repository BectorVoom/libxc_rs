//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 725/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk725(t4243: f64, t552: f64, t573: f64, t1466: f64, t1527: f64, t1535: f64, t1529: f64, t1539: f64, t4121: f64, t569: f64, t4124: f64, t556: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4244 = t4243 * t552;
    let t4245 = t4244 * sigma2;
    let t4246 = t4245 * t573;
    let t4248 = t1527 * t1466;
    let t4249 = t4248 * sigma2;
    let t4250 = t4249 * t1535;
    let t4252 = t1529 * t1539;
    let t4254 = t569 * t4121;
    let t4255 = t4254 * sigma2;
    let t4256 = t556 * t4124;
    (t4245, t4246, t4248, t4249, t4250, t4252, t4254, t4255, t4256)
}
