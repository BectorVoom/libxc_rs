//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1779/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1779(t22829: f64, t80958: f64, t2229: f64, t583: f64, t60: f64, t1995: f64, t22816: f64, t22818: f64, t22765: f64, t3858: f64, t22764: f64, t3777: f64) -> (f64, f64, f64, f64, f64) {
    let t80959 = t80958 * t22829;
    let t80967 = 1.0_f64 / t60 / t2229 / t583;
    let t80970 = t80967 * t1995 * t22816 * t22818;
    let t80989 = t22765 * t3858;
    let t80991 = t3777 * t22764;
    (t80959, t80967, t80970, t80989, t80991)
}
