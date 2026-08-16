//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1119/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1119(t14613: f64, t14657: f64, t1055: f64, t10160: f64, t10170: f64, t1052: f64, t1066: f64, t11010: f64, t14545: f64, t14549: f64, t14552: f64, t14555: f64, t14562: f64, t1635: f64, t3169: f64, t3176: f64, t3207: f64, t388: f64, t4557: f64, t4660: f64, t4665: f64) -> f64 {
    let t14658 = t14613 + t14657;
    let t14659 = t1055 * t14658;
    let t14661 = -2.0_f64 * t10160 * t1635 - t10170 * t1635 + 2.0_f64 * t1052 * t14549 - t1052 * t14659 - 2.0_f64 * t1066 * t14545 - 2.0_f64 * t1066 * t14552 - 2.0_f64 * t1066 * t14555 - t11010 * t1635 + 2.0_f64 * t14562 * t388 + 4.0_f64 * t3169 * t4665 + 2.0_f64 * t3176 * t4557 + 2.0_f64 * t3176 * t4660 - t3207 * t4557;
    t14661
}
