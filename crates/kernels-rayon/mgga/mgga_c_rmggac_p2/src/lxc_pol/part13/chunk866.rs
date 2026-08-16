//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 866/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk866(t1971: f64, t30900: f64, t3351: f64, t4617: f64, t2310: f64, t34881: f64, t35384: f64, t2313: f64, t34855: f64, t674: f64, t7411: f64, t1240: f64, t236: f64, t618: f64, t7230: f64, t7231: f64) -> (f64, f64, f64, f64, f64) {
    let t39293 = t3351 * t1971 * t4617 * t30900;
    let t39295 = t34881 * t2310;
    let t39297 = t35384 * t2310;
    let t39300 = t2313 * t34855 * t674;
    let t39301 = t39300 * t7411;
    let t39306 = t7230 * t7231 * t236 * t618 * t1240;
    (t39293, t39295, t39297, t39301, t39306)
}
