//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1258/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1258(t33: f64, t259: f64, t479: f64, t21701: f64, t1289: f64, t1826: f64, t21741: f64, t4579: f64, t57: f64, t6393: f64, t21709: f64, t1791: f64, t21165: f64, t1675: f64, t1792: f64, t18648: f64, t18666: f64, t19349: f64, t20246: f64, t20255: f64, t20257: f64, t20264: f64, t20276: f64, t20278: f64, t21116: f64, t21123: f64, t21129: f64, t21133: f64, t21136: f64, t21139: f64, t21146: f64, t5785: f64, t6073: f64, t6077: f64, t6080: f64, t6304: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t21742 = piecewise3(t480, 0.0_f64, t21701);
    let t21749 = piecewise3(t386, t21741, t21742 * t57 / 2.0_f64 - t6393 * t1289 - t1826 * t4579 / 2.0_f64);
    let t21750 = t21709 + t21749;
    let t21756 = t1791 * t21165;
    let t21784 = 80.0_f64 / 9.0_f64 * t20257 + t18648 + t1675 * t21756 / 3.0_f64 + 20.0_f64 / 3.0_f64 * t19349 * t20264 + 10.0_f64 * t18666 * t21116 - 16.0_f64 / 9.0_f64 * t20276 + t21146 * t1792 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t6073 * t6304 - 16.0_f64 / 9.0_f64 * t20278 + 32.0_f64 / 9.0_f64 * t20255 - 10.0_f64 / 3.0_f64 * t20246 * t6077 - 4.0_f64 / 3.0_f64 * t21123 * t1792 - 10.0_f64 / 3.0_f64 * t5785 * t21129 - 5.0_f64 / 3.0_f64 * t5785 * t21133 - 2.0_f64 / 3.0_f64 * t21136 * t1792 - 2.0_f64 / 3.0_f64 * t21139 * t1792 - 4.0_f64 / 3.0_f64 * t6080 * t6304;
    (t21742, t21750, t21756, t21784)
}
