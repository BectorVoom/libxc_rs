//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 653/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk653(t734: f64, t9058: f64, t716: f64, t8664: f64, t740: f64, t748: f64, t5330: f64, t8780: f64, t746: f64, t741: f64, t2579: f64, t2586: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9059 = t734 * t9058;
    let t9061 = t8664 * t716;
    let t9062 = t9061 * t740;
    let t9063 = t9062 * t748;
    let t9065 = t5330 * t8780;
    let t9066 = t746 * t9065;
    let t9067 = t741 * t9066;
    let t9069 = t2586 * t2579;
    (t9059, t9061, t9062, t9063, t9065, t9066, t9067, t9069)
}
