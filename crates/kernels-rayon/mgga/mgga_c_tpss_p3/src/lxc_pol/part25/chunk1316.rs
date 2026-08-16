//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1316/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1316(t17930: f64, t51780: f64, t21262: f64, t60960: f64, t4706: f64, t580: f64, t14256: f64, t19671: f64, t4806: f64, t750: f64, t19817: f64, t1398: f64, t555: f64, t64300: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t69820 = t17930 * t51780;
    let t69828 = t60960 * t21262;
    let t69838 = t580 * t4706;
    let t69842 = t19671 * t14256;
    let t69847 = t4806 * t750;
    let t69848 = t19817 * t69847;
    let t69855 = t64300 * t555 * t1398;
    (t69820, t69828, t69838, t69842, t69847, t69848, t69855)
}
