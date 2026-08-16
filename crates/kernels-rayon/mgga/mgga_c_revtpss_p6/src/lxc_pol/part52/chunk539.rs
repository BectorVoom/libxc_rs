//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 539/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk539(t213: f64, t4503: f64, t2783: f64, t1568: f64, t233: f64, t869: f64, t689: f64, t72: f64, t686: f64, t874: f64, t822: f64, t1559: f64, t234: f64, t2776: f64, t2780: f64, t2787: f64, t2791: f64, t2796: f64, t2802: f64, t2806: f64, t2810: f64, t2815: f64, t4366: f64, t4424: f64, t4469: f64, t4494: f64, t4497: f64, t4501: f64, t820: f64, t837: f64, t879: f64) -> f64 {
    let t4504 = t213 * t4503;
    let t4514 = t213 * t2783;
    let t4518 = t233 * t1568;
    let t4519 = t869 * t4518;
    let t4520 = t689 * t4519;
    let t4522 = t1568 * t72;
    let t4524 = t874 * t4522 * t686;
    let t4526 = t822 * t1568;
    let t4533 = t2776 - t2780 + 0.54878743191129263322e-2_f64 * t2787 - 0.54878743191129263322e-2_f64 * t2791 + t2796 - 0.9757440539382783019e-2_f64 * t2802 + 0.9757440539382783019e-2_f64 * t2806 - t2810 + 0.54878743191129263322e-2_f64 * t4497 - 0.9757440539382783019e-2_f64 * t4501 + 0.13170898365871023197e1_f64 * t4504 * t4494 * t4366 - 0.65854491829355115987e0_f64 * t820 * t2815 * t1559 - 0.65854491829355115987e0_f64 * t820 * t879 * t4424 - 0.65854491829355115987e0_f64 * t4514 * t4494 * t837 - 0.54878743191129263322e-2_f64 * t4520 + 0.9757440539382783019e-2_f64 * t4524 - 0.65854491829355115987e0_f64 * t820 * t4526 * t837 + 0.65854491829355115987e0_f64 * t213 * t234 * t4469;
    t4533
}
