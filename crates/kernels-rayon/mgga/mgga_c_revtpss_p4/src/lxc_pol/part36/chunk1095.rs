//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1095/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1095(t24839: f64, t3720: f64, t24232: f64, t247: f64, t3618: f64, t1264: f64, t24248: f64, t1222: f64, t1261: f64, t12809: f64, t12855: f64, t1808: f64, t21242: f64, t24817: f64, t24821: f64, t24827: f64, t24831: f64, t24836: f64, t5373: f64, t5381: f64, t5391: f64, t6653: f64, t6673: f64, t6679: f64, t6683: f64) -> (f64, f64, f64, f64) {
    let t24840 = t3720 * t24839;
    let t24846 = t247 * t3618 * t24232;
    let t24858 = t247 * t1264 * t24248;
    let t24861 = -t1222 * t24817 / 288.0_f64 - t1222 * t24821 / 48.0_f64 - t5373 * t6653 / 27.0_f64 - 7.0_f64 / 648.0_f64 * t1222 * t24827 + t1222 * t24831 / 36.0_f64 - 0.12862205435420921092e-2_f64 * t12855 * t24836 + 0.64311027177104605458e-3_f64 * t12809 * t24840 + 0.7145669686344956162e-3_f64 * t5381 * t6673 + 0.14291339372689912324e-2_f64 * t1261 * t24846 + 0.45732285992607719436e-2_f64 * t21242 * t1808 + 0.22866142996303859718e-2_f64 * t5391 * t6679 + 0.45732285992607719436e-2_f64 * t5391 * t6683 - 0.42874018118069736972e-3_f64 * t5381 * t6679 - 0.14291339372689912324e-3_f64 * t1261 * t24858;
    (t24840, t24846, t24858, t24861)
}
