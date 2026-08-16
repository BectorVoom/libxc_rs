//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 276/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk276(t179: f64, t824: f64, t932: f64, t385: f64, t404: f64, t906: f64, t909: f64, t918: f64, t923: f64, t929: f64) -> (f64, f64) {
    let t934 = t179 * t932 * t824;
    let t937 = t906 - t385 * t909 / 96.0_f64 + 0.21437009059034868486e-3_f64 * t918 * t923 + t929 - 0.42874018118069736972e-3_f64 * t404 * t934;
    (t934, t937)
}
