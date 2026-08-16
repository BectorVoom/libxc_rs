//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 713/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk713(t3236: f64, t3245: f64, t1032: f64, t2689: f64, t1001: f64, t3271: f64, t982: f64, t12652: f64, t12654: f64, t12656: f64, t12660: f64, t12665: f64, t12667: f64, t12669: f64, t12672: f64, t12675: f64, t12678: f64, t12683: f64) -> (f64, f64, f64, f64) {
    let t12685 = t3236 * t3245;
    let t12687 = t1032 * t2689;
    let t12689 = t3271 * t1001;
    let t12690 = t982 * t12689;
    let t12692 = t12652 / 8.0_f64 - 3.0_f64 * t12654 - 3.0_f64 / 4.0_f64 * t12656 + 3.0_f64 / 4.0_f64 * t12660 - 3.0_f64 / 32.0_f64 * t12665 + 3.0_f64 / 16.0_f64 * t12667 - 15.0_f64 / 16.0_f64 * t12669 - 3.0_f64 / 32.0_f64 * t12672 - 3.0_f64 / 8.0_f64 * t12675 - 3.0_f64 / 2.0_f64 * t12678 + 15.0_f64 / 8.0_f64 * t12683 + 3.0_f64 / 2.0_f64 * t12685 + 9.0_f64 / 4.0_f64 * t12687 + 15.0_f64 / 16.0_f64 * t12690;
    (t12685, t12687, t12690, t12692)
}
