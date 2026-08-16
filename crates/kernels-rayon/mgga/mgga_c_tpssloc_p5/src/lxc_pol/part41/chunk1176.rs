//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1176/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1176(t1136: f64, t6037: f64, t1683: f64, t4819: f64, t6056: f64, t6053: f64, t3359: f64, t6052: f64, t4823: f64, t11352: f64, t6036: f64, t11137: f64, t11444: f64, t14702: f64, t14720: f64, t15194: f64, t15195: f64, t18203: f64, t18208: f64, t18213: f64, t18217: f64, t18219: f64, t18223: f64, t18227: f64, t18229: f64, t18234: f64, t18239: f64, t18243: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18631 = t6037 * t1136;
    let t18634 = t1683 * t4819;
    let t18637 = t6056 * t1136;
    let t18640 = t6053 * t1136;
    let t18643 = t6052 * t3359;
    let t18644 = t18643 * t1136;
    let t18647 = t4823 * t4819;
    let t18650 = t6036 * t11352;
    let t18651 = t18650 * t1136;
    let t18668 = -t11444 + 0.76103703703703703703e-2_f64 * t11137 + 0.1522074074074074074e-1_f64 * t14702 + 0.761037037037037037e-2_f64 * t14720 - t15194 - t15195 + 0.3805185185185185185e-2_f64 * t18203 + 0.19025925925925925925e-1_f64 * t18208 - 0.68493333333333333331e-1_f64 * t18213 - 0.2283111111111111111e-1_f64 * t18217 - 0.11415555555555555555e-1_f64 * t18219 + 0.10274e0_f64 * t18223 + 0.68493333333333333332e-1_f64 * t18227 - 0.57077777777777777777e-2_f64 * t18229 - 0.11415555555555555555e-1_f64 * t18234 + 0.34246666666666666666e-1_f64 * t18239 + 0.17123333333333333333e-1_f64 * t18243;
    (t18631, t18634, t18637, t18640, t18644, t18647, t18651, t18668)
}
