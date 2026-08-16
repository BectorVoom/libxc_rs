//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 489/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk489(t2822: f64, t699: f64, t909: f64, t241: f64, t976: f64, t891: f64, t275: f64, t290: f64, t2764: f64, t919: f64, t923: f64, t307: f64, t922: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2823 = 0.13692777777777777778e0_f64 * t2822;
    let t2824 = t699 * t909;
    let t2826 = t241 * t976;
    let t2840 = t891 * t891;
    let t2841 = 1.0_f64 / t2840;
    let t2842 = t275 * t2841;
    let t2843 = t290 * t290;
    let t2844 = 1.0_f64 / t2843;
    let t2848 = 0.22831111111111111111e-1_f64 * t2764;
    let t2856 = t919 * t923;
    let t2859 = t922 * t307;
    (t2823, t2824, t2826, t2842, t2844, t2848, t2856, t2859)
}
