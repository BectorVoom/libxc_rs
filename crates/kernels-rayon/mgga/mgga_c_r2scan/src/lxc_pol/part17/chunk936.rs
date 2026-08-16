//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 936/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk936(t10856: f64, t2158: f64, t2111: f64, t2164: f64, t6190: f64, t1050: f64, t120: f64, t6239: f64, t269: f64, t787: f64) -> (f64, f64, f64, f64) {
    let t10857 = t10856 * t2158;
    let t10863 = t2111 * t6190 * t2164;
    let t10866 = t120 * t6239 * t1050;
    let t10868 = t787 * t269;
    (t10857, t10863, t10866, t10868)
}
