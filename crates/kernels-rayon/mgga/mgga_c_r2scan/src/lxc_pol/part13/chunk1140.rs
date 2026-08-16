//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1140/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1140(t10814: f64, t2651: f64, t10698: f64, t2593: f64, t11805: f64, t37641: f64, t1054: f64, t5108: f64, t7352: f64, t37759: f64, t37749: f64, t37762: f64, t39655: f64, t39658: f64, t39661: f64, t39664: f64, t39667: f64) -> f64 {
    let t39669 = t2651 * t10814;
    let t39672 = t10698 * t2593;
    let t39673 = 0.25610080155860322884e0_f64 * t39672;
    let t39674 = t37641 * t11805;
    let t39677 = t5108 * t1054 * t7352;
    let t39679 = 0.11902492299418487743e0_f64 * t37759;
    let t39681 = -0.65495539973149862688e-2_f64 * t39655 + 0.43663693315433241792e-2_f64 * t39658 - 0.13002332610081402845e0_f64 * t39661 - 0.17336443480108537126e0_f64 * t39664 + 0.65495539973149862688e-2_f64 * t39667 - 0.43341108700271342816e-1_f64 * t39669 - 0.69345773920434148506e0_f64 * t37749 - t39673 + 0.2600466522016280569e0_f64 * t39674 - 0.2600466522016280569e0_f64 * t39677 - t39679 + 0.23804984598836975486e-2_f64 * t37762;
    t39681
}
