//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 854/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk854(t2060: f64, t3033: f64, t2062: f64, t2823: f64, t7902: f64, t4695: f64, t4703: f64, t4880: f64, t4891: f64, t6946: f64, t6948: f64, t6951: f64, t8545: f64, t8547: f64) -> f64 {
    let t9033 = t2060 * t3033;
    let t9034 = t9033 * t2062;
    let t9036 = t2823 * t7902;
    let t9038 = -t4695 - t4880 + t6946 - t8545 - 0.675260332e-1_f64 * t9034 - 0.1350520664e0_f64 * t9036 + t6948 + t4891 - t4703 - t6951 - t8547;
    t9038
}
