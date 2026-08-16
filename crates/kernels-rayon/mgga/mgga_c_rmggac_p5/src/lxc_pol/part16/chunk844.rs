//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 844/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk844(t7244: f64, t8422: f64, t2310: f64, t7939: f64, t2283: f64, t38354: f64, t7473: f64, t118: f64, t2281: f64, t498: f64, t7418: f64, t9153: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41828 = t7244 * t8422;
    let t41882 = t7939 * t2310;
    let t41884 = t7939 * t2283;
    let t41890 = t38354 * t7473;
    let t41914 = t7418 * t118 * t2281 * t498;
    let t41922 = t7244 * t9153;
    (t41828, t41882, t41884, t41890, t41914, t41922)
}
