//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1452/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1452(t14933: f64, t449: f64, t300: f64, t1671: f64, t3265: f64, t3313: f64, t14722: f64, t14704: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t11459: f64, t14702: f64, t14708: f64, t14720: f64, t14728: f64, t14733: f64, t14738: f64, t14742: f64, t14746: f64, t14751: f64, t14755: f64) -> (f64, f64, f64, f64) {
    let t14934 = t14933 * t449;
    let t14936 = 0.19751673498613801407e-1_f64 * t300 * t14934;
    let t14937 = t1671 * t3265;
    let t14939 = 6.0_f64 * t3313 * t14937;
    let t14946 = 0.23744444444444444444e-1_f64 * t14722;
    let t14947 = 0.11872222222222222222e-1_f64 * t14704;
    let t14956 = -t11459 + 0.15829629629629629629e-1_f64 * t11137 + 0.39574074074074074073e-2_f64 * t11139 - 0.11872222222222222222e-1_f64 * t11141 - 0.5936111111111111111e-2_f64 * t11143 + 0.79148148148148148146e-2_f64 * t14702 + 0.79148148148148148146e-2_f64 * t14720 - t14946 - t14947 + 0.19787037037037037037e-1_f64 * t14728 - 0.71233333333333333332e-1_f64 * t14733 - 0.23744444444444444444e-1_f64 * t14738 - 0.11872222222222222222e-1_f64 * t14742 + 0.10685e0_f64 * t14746 + 0.71233333333333333332e-1_f64 * t14751 + 0.35616666666666666666e-1_f64 * t14755 + 0.17808333333333333333e-1_f64 * t14708;
    (t14934, t14936, t14939, t14956)
}
