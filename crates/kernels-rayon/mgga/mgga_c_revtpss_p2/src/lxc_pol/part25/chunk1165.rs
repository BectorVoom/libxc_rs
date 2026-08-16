//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1165/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1165(t2430: f64, t33: f64, t2408: f64, t1113: f64, t890: f64, t2832: f64, t1940: f64, t1963: f64, t2403: f64, t25206: f64, t25436: f64, t25440: f64, t25445: f64, t25752: f64, t25760: f64, t25763: f64, t3351: f64, t4541: f64, t7087: f64, t7091: f64, t7200: f64, t7207: f64) -> (f64, f64, f64, f64, f64) {
    let t25767 = t33 * t2430;
    let t25778 = t33 * t2408;
    let t25781 = t1113 * t890;
    let t25784 = t33 * t2832;
    let t25791 = 3.0_f64 * t4541 * t1963 * t25752 + 3.0_f64 * t2403 * t7087 * t7200 - 3.0_f64 * t25206 * t25760 + 3.0_f64 * t2403 * t1963 * t25763 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t25767 + t1940 * t25436 * t33 / 2.0_f64 - t1940 * t25440 * t7207 + t1940 * t7087 * t1113 + t1940 * t25445 * t25778 - t1940 * t7091 * t25781 - t1940 * t7091 * t25784 / 2.0_f64 + t1940 * t1963 * t3351 / 2.0_f64;
    (t25767, t25778, t25781, t25784, t25791)
}
