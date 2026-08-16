//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 500/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk500(t2843: f64, t2764: f64, t919: f64, t923: f64, t307: f64, t922: f64, t302: f64, t2822: f64, t310: f64, t938: f64, t942: f64, t320: f64, t941: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2844 = 1.0_f64 / t2843;
    let t2848 = 0.22831111111111111111e-1_f64 * t2764;
    let t2856 = t919 * t923;
    let t2859 = t922 * t307;
    let t2860 = 1.0_f64 / t2859;
    let t2861 = t302 * t2860;
    let t2868 = 0.68863333333333333333e0_f64 * t2764;
    let t2875 = 0.17365833333333333333e0_f64 * t2822;
    let t2884 = t922 * t922;
    let t2885 = 1.0_f64 / t2884;
    let t2886 = t302 * t2885;
    let t2887 = t310 * t310;
    let t2888 = 1.0_f64 / t2887;
    let t2892 = 0.12361111111111111111e-1_f64 * t2764;
    let t2900 = t938 * t942;
    let t2903 = t941 * t320;
    (t2844, t2848, t2856, t2861, t2868, t2875, t2886, t2888, t2892, t2900, t2903)
}
