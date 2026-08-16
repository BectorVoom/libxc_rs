//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2333/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2333(t27700: f64, t95588: f64, t18975: f64, t7345: f64, t18332: f64, t7310: f64, t1222: f64, t29606: f64, t1748: f64, t18584: f64, t24741: f64, t27580: f64, t27604: f64, t27655: f64, t27687: f64, t27714: f64, t5030: f64, t6232: f64, t7999: f64, t8031: f64, t8035: f64, t86167: f64, t95452: f64, t95662: f64, t95702: f64) -> f64 {
    let t104425 = t95588 * t27700;
    let t104435 = t7345 * t18975;
    let t104441 = t7310 * t18332;
    let t104445 = t29606 * t1222;
    let t104449 = -0.16149102437656156342e-2_f64 * t104425 - t95662 - 0.16149102437656156342e-2_f64 * t27580 * t8035 + 0.20186378047070195428e-3_f64 * t27714 * t8035 + 0.20186378047070195428e-3_f64 * t8031 * t27655 - t86167 * t6232 / 1536.0_f64 + 5.0_f64 / 10368.0_f64 * t104435 + t95452 * t1748 / 216.0_f64 + t27604 * t5030 / 216.0_f64 + t104441 / 648.0_f64 + t7999 * t27687 / 27.0_f64 + t104445 / 2304.0_f64 - t24741 * t18584 / 1152.0_f64 + t95702;
    t104449
}
