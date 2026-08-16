//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 866/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk866(t4721: f64, t4901: f64, t4964: f64, t4967: f64, t4972: f64, t4975: f64, t4979: f64, t4981: f64, t6954: f64, t6960: f64, t7861: f64, t2049: f64, t759: f64, t955: f64) -> (f64, f64) {
    let t7862 = -t4901 + t7861 - t4721 + t4964 - t4967 - t6954 - t4972 + t4975 - t6960 + t4979 + t4981;
    let t7865 = t759 * t955 * t2049;
    (t7862, t7865)
}
