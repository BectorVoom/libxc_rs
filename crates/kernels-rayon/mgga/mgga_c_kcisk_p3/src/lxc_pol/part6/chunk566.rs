//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 566/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk566(t3592: f64, t7877: f64, t457: f64, t3611: f64, t5668: f64, t7738: f64, t7742: f64, t7746: f64, t7758: f64, t7765: f64, t1355: f64, t2083: f64, t306: f64, t3599: f64, t5687: f64, t7757: f64, t7764: f64) -> (f64, f64, f64, f64) {
    let t7878 = t3592 * t7877;
    let t7879 = t457 * t7878;
    let t7894 = -0.991e-2_f64 * t7758 + 0.1982e-1_f64 * t7765 + t3611 + 0.27516666666666666666e-2_f64 * t5668 - 0.27516666666666666667e-2_f64 * t7738 + 0.8255e-2_f64 * t7742 - 0.41275e-2_f64 * t7746;
    let t7897 = -t3599 * t7757 / 8.0_f64 + t5687 * t2083 / 2.0_f64 + t1355 * t7764 / 4.0_f64 + t306 * t7894 / 2.0_f64;
    (t7878, t7879, t7894, t7897)
}
