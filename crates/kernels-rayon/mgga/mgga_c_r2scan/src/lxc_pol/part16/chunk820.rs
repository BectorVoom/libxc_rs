//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 820/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk820(t471: f64, t8629: f64, t97: f64, t4695: f64, t4703: f64, t4721: f64, t4880: f64, t4891: f64, t4901: f64, t4964: f64, t4967: f64, t6946: f64, t6948: f64, t6951: f64, t8545: f64, t8547: f64, t8552: f64, t8555: f64, t8556: f64) -> (f64, f64) {
    let t8631 = t97 * t471 * t8629;
    let t8632 = 3.0_f64 * t8631;
    let t8633 = t4695 + t4880 - t6946 + t8545 - t6948 - t4891 + t4703 + t6951 + t8547 - t8552 + t4901 - t8555 + t4721 - t4964 + t4967 + t8556;
    (t8632, t8633)
}
