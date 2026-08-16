//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2235/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2235(t17303: f64, t7613: f64, t29062: f64, t3678: f64, t17209: f64, t26880: f64, t29019: f64, t3707: f64, t26873: f64, t5265: f64, t15687: f64, t26865: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t104825 = t7613 * t17303;
    let t104828 = 0.30488190661738479624e-2_f64 * t29062 * t3678;
    let t104833 = 0.3811023832717309953e-3_f64 * t26880 * t17209;
    let t104834 = t3707 * t29019;
    let t104844 = 0.57165357490759649296e-3_f64 * t26873 * t5265;
    let t104852 = t26865 * t15687;
    (t104825, t104828, t104833, t104834, t104844, t104852)
}
