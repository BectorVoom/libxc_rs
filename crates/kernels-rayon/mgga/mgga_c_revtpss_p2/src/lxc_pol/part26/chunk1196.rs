//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1196/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1196(t1113: f64, t1940: f64, t2071: f64, t2403: f64, t25752: f64, t25760: f64, t25763: f64, t25778: f64, t25781: f64, t26581: f64, t26585: f64, t3351: f64, t4541: f64, t7200: f64, t7207: f64, t7428: f64, t7432: f64, t9357: f64, t94255: f64, t94262: f64, t94286: f64, t94312: f64, t94320: f64, t95511: f64, t95527: f64, t95964: f64, t95976: f64) -> f64 {
    let t96166 = 9.0_f64 * t4541 * t2071 * t94262 + 9.0_f64 / 2.0_f64 * t2403 * t26581 * t7200 + 9.0_f64 * t2403 * t7428 * t25763 + 3.0_f64 / 2.0_f64 * t1940 * t26581 * t1113 - 9.0_f64 * t95511 * t25760 + t1940 * t2071 * t9357 / 2.0_f64 - 3.0_f64 * t1940 * t95964 * t94312 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t94320 - 3.0_f64 * t1940 * t26585 * t25781 - 3.0_f64 / 2.0_f64 * t1940 * t7432 * t94286 + 3.0_f64 * t1940 * t95976 * t25778 + 9.0_f64 * t4541 * t7428 * t25752 + 3.0_f64 / 2.0_f64 * t1940 * t7428 * t3351 - t1940 * t7432 * t94255 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t1940 * t95527 * t7207;
    t96166
}
