//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1107/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1107(t2887: f64, t2890: f64, t487: f64, t2003: f64, t2888: f64, t178: f64, t17933: f64, t17930: f64, t18000: f64, t18009: f64, t2064: f64, t2899: f64, t2902: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21359 = t2887 * t487 * t2890;
    let t21360 = t21359 / 72.0_f64;
    let t21395 = t2888 * t2003;
    let t21454 = t17933 * t178;
    let t21455 = t17930 * t21454;
    let t21462 = t18000 * t21454;
    let t21468 = t18009 * t21454;
    let t21499 = t2899 * t2064 * t2902;
    (t21360, t21395, t21454, t21455, t21462, t21468, t21499)
}
