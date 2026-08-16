//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1035/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1035(t23842: f64, t4806: f64, t1042: f64, t23633: f64, t4801: f64, t1651: f64, t5825: f64, t4872: f64, t1592: f64, t19649: f64, t1015: f64, t22671: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23843 = t4806 * t23842;
    let t23844 = t1042 * t23843;
    let t23847 = t4806 * t23633;
    let t23848 = t1042 * t23847;
    let t23851 = t4801 * t23842;
    let t23852 = t1042 * t23851;
    let t23857 = t5825 * t1651;
    let t23858 = t4872 * t23857;
    let t23859 = t1042 * t23858;
    let t23862 = t19649 * t1592;
    let t23863 = t1042 * t23862;
    let t23868 = t1015 * t22671;
    (t23844, t23848, t23852, t23859, t23863, t23868)
}
