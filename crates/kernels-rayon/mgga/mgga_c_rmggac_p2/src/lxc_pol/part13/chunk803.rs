//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 803/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk803(t35565: f64, t35607: f64, t35611: f64, t35616: f64, t35618: f64, t35621: f64, t35696: f64, t35698: f64, t35702: f64, t35712: f64, t35716: f64, t35728: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37768 = 0.487802396665200453e-2_f64 * t35565;
    let t37786 = 0.91462949374725084936e-3_f64 * t35607;
    let t37787 = 0.487802396665200453e-2_f64 * t35611;
    let t37788 = 0.11709622077411463733e-2_f64 * t35616;
    let t37789 = 0.18292589874945016987e-2_f64 * t35618;
    let t37790 = 0.26021382394247697185e-3_f64 * t35621;
    let t37815 = 0.89430439388620083049e-2_f64 * t35696;
    let t37816 = 0.487802396665200453e-2_f64 * t35698;
    let t37818 = 0.18292589874945016987e-2_f64 * t35702;
    let t37821 = 0.18292589874945016987e-2_f64 * t35712;
    let t37822 = 0.26021382394247697185e-3_f64 * t35716;
    let t37825 = 0.13010691197123848592e-3_f64 * t35728;
    (t37768, t37786, t37787, t37788, t37789, t37790, t37815, t37816, t37818, t37821, t37822, t37825)
}
