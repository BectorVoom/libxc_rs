//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 866/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk866(t4703: f64, t4880: f64, t4891: f64, t6943: f64, t6946: f64, t6947: f64, t6948: f64, t6949: f64, t6950: f64, t6951: f64, t6952: f64, t2461: f64, t759: f64, t761: f64) -> (f64, f64) {
    let t7858 = -t6943 - t4880 + t6946 - t6947 - t6948 + t4891 + t6949 + t6950 - t4703 + t6951 + t6952;
    let t7861 = 0.571528e-1_f64 * t759 * t2461 * t761;
    (t7858, t7861)
}
