//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1239/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1239(t35260: f64, t35271: f64, t37458: f64, t39797: f64, t39802: f64, t39807: f64, t39809: f64, t39811: f64, t39813: f64, t39815: f64, t39817: f64, t39819: f64, t39822: f64, t39825: f64, t39829: f64, t39831: f64, t39833: f64, t39835: f64) -> f64 {
    let t41815 = t37458 + 0.12862205435420921092e-2_f64 * t39797 - 0.75475421495049964965e-2_f64 * t35260 - 0.62896184579208304137e-2_f64 * t39802 - 0.37737710747524982482e-1_f64 * t39807 - 0.34299214494455789578e-2_f64 * t39809 - 0.13719685797782315831e-1_f64 * t39811 + 0.68598428988911579156e-2_f64 * t39813 - 0.10289764348336736873e-1_f64 * t39815 + 0.34299214494455789578e-2_f64 * t39817 + 0.51448821741683684367e-2_f64 * t39819 + t39822 / 12.0_f64 + t39825 / 96.0_f64 + 0.94344276868812456205e-2_f64 * t39829 - 0.34299214494455789578e-2_f64 * t39831 + 0.17149607247227894789e-2_f64 * t39833 + 0.32012600194825403606e-1_f64 * t39835 - 0.42874018118069736972e-3_f64 * t35271;
    t41815
}
