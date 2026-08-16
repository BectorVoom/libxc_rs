//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 684/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk684(t10660: f64, t1653: f64, t571: f64, t1648: f64, t4624: f64, t1646: f64, t574: f64, t581: f64, t4663: f64, t4652: f64, t4716: f64, t10579: f64, t10582: f64, t10590: f64, t10598: f64, t10639: f64, t10642: f64, t10644: f64, t10647: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10661 = t1653 * t10660;
    let t10663 = 1.0_f64/pow_3_2(t571);
    let t10664 = t4624 * t1648;
    let t10665 = t10663 * t10664;
    let t10667 = t1646 * t10660;
    let t10671 = 1.0_f64 / t574 / t581 / 4.0_f64;
    let t10672 = t10671 * t10664;
    let t10674 = t4663 * t1648;
    let t10675 = t10674 * t4652;
    let t10677 = t4716 * t1648;
    let t10678 = t10677 * t4652;
    let t10680 = -0.33547222222222222222e0_f64 * t10579 + 0.12077e1_f64 * t10582 - 0.181155e1_f64 * t10590 - 0.301925e0_f64 * t10598 - t10639 - t10642 - 0.82785e-1_f64 * t10644 + 0.49671e0_f64 * t10647 + 0.16504875e0_f64 * t10661 - 0.412621875e-1_f64 * t10665 + 0.258925e1_f64 * t10667 + 0.19419375e1_f64 * t10672 - 0.3883875e1_f64 * t10675 + 0.247573125e0_f64 * t10678;
    (t10661, t10664, t10665, t10667, t10671, t10672, t10675, t10678, t10680)
}
