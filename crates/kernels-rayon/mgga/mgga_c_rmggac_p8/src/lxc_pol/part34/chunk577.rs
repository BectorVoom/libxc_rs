//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 577/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk577(t14252: f64, t14256: f64, t14259: f64, t2020: f64, t3180: f64, t2019: f64, t2604: f64, t3188: f64, t14494: f64, t515: f64, t235: f64, t14375: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14693 = 0.23268647941669485538e-4_f64 * t14252;
    let t14694 = 0.58171619854173713846e-5_f64 * t14256;
    let t14695 = 0.58171619854173713846e-5_f64 * t14259;
    let t14696 = t2020 * t3180;
    let t14697 = t2019 * t14696;
    let t14701 = t2604 * t3188;
    let t14702 = 0.14967802127329760705e-1_f64 * t14701;
    let t14703 = t515 * t14494;
    let t14704 = t235 * t14703;
    let t14705 = 0.19957069503106347607e-1_f64 * t14704;
    let t14709 = 0.1276937996798935182e-4_f64 * t14375;
    (t14693, t14694, t14695, t14696, t14697, t14702, t14703, t14705, t14709)
}
