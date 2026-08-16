//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 948/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk948(t1055: f64, t2080: f64, t2111: f64, t2164: f64, t6190: f64, t1050: f64, t120: f64, t6239: f64, t269: f64, t787: f64) -> (f64, f64, f64, f64) {
    let t10860 = t2080 * t1055;
    let t10863 = t2111 * t6190 * t2164;
    let t10864 = 0.14457274399185490173e-3_f64 * t10863;
    let t10866 = t120 * t6239 * t1050;
    let t10867 = 0.21341733463216935736e0_f64 * t10866;
    let t10868 = t787 * t269;
    (t10860, t10864, t10867, t10868)
}
