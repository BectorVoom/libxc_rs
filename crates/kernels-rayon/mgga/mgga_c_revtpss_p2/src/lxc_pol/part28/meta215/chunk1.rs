//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1018/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1018(t366: f64, t4797: f64, t1065: f64, t2857: f64, t4181: f64, t1042: f64) -> (f64, f64, f64, f64) {
    let t4798 = t4797 * t366;
    let t4801 = t1065 * t2857;
    let t4802 = t4801 * t4181;
    let t4803 = t1042 * t4802;
    (t4798, t4801, t4802, t4803)
}
