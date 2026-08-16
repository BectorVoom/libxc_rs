//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1023/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1023(t322: f64, t12828: f64, t1120: f64, t2944: f64, t1013: f64, t3730: f64, t2941: f64, t11223: f64, t12244: f64, t1300: f64, t327: f64, t3509: f64, t6693: f64, t834: f64) -> (f64, f64, f64, f64, f64) {
    let t324 = 0.0_f64 < t322;
    let t12829 = piecewise3(t324, 0.0_f64, t12828);
    let t12838 = t1120 * t2944;
    let t12841 = t3730 * t1013;
    let t12844 = t1120 * t2941;
    let t12849 = -0.64e0_f64 * t12829 * t327 - 0.256e1_f64 * t12244 * t1013 - 0.384e1_f64 * t11223 * t2944 - 0.128e1_f64 * t3509 * t2941 - 0.384e1_f64 * t6693 * t12838 - 0.256e1_f64 * t1300 * t12841 - 0.128e1_f64 * t1300 * t12844 - 0.64e0_f64 * t834 * t12829;
    (t12829, t12838, t12841, t12844, t12849)
}
