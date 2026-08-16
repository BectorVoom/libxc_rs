//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1260/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1260(t21804: f64, t1838: f64, t5432: f64, t18490: f64, t18967: f64, t21074: f64, t1656: f64, t6419: f64, t5740: f64, t5448: f64, t5380: f64, t18511: f64, t3260: f64, param_beta: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21805 = param_beta * t21804;
    let t21819 = t1838 * t5432;
    let t21820 = t18490 * t21819;
    let t21823 = t18967 * t21074;
    let t21826 = t6419 * t1656;
    let t21827 = t5740 * t21826;
    let t21830 = t1838 * t5448;
    let t21831 = t5740 * t21830;
    let t21834 = t1838 * t5380;
    let t21836 = t18511 * t21834 * t3260;
    (t21805, t21819, t21820, t21823, t21826, t21827, t21830, t21831, t21834, t21836)
}
