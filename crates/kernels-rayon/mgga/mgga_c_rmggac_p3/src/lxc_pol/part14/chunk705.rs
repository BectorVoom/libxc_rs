//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 705/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk705(t8356: f64, t8467: f64, t8470: f64, t8477: f64, t8484: f64, t8488: f64, t8492: f64, t8534: f64, t8657: f64, t8820: f64, t9037: f64, t9069: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9730 = 0.30487649791575028314e-3_f64 * t8356;
    let t9743 = 0.72042316457491791906e-3_f64 * t8467;
    let t9744 = 0.10248087766267884742e-3_f64 * t8470;
    let t9758 = 0.30487649791575028314e-3_f64 * t8477;
    let t9759 = 0.43368970657079495312e-4_f64 * t8484;
    let t9760 = 0.30487649791575028314e-3_f64 * t8488;
    let t9761 = 0.43368970657079495312e-4_f64 * t8492;
    let t9768 = 0.18183107769496894486e-1_f64 * t8534;
    let t9947 = 0.18183107769496894486e-1_f64 * t8657;
    let t10035 = 2.0_f64 * t8820;
    let t10060 = 0.24829349937757072982e-4_f64 * t9037;
    let t10061 = 0.4726e1_f64 * t9069;
    (t9730, t9743, t9744, t9758, t9759, t9760, t9761, t9768, t9947, t10035, t10060, t10061)
}
