//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2067/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2067(t1539: f64, t6746: f64, t82655: f64, t14220: f64, t7581: f64, t25555: f64, t82822: f64, t25529: f64, t6680: f64, t1920: f64, t2966: f64, t7614: f64) -> (f64, f64, f64, f64, f64) {
    let t89395 = t82655 * t1539 * t6746;
    let t89399 = t82655 * t7581 * t14220;
    let t89421 = 0.18277045187202515961e-2_f64 * t82822 * t25555;
    let t89429 = 0.14621636149762012769e-1_f64 * t6680 * t25529;
    let t89431 = t1920 * t2966 * t7614;
    (t89395, t89399, t89421, t89429, t89431)
}
