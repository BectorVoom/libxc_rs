//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 891/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk891(t5: f64, t12571: f64, t8511: f64, t1437: f64, t8513: f64, t8514: f64, t1409: f64, t31682: f64, t8308: f64, t1433: f64, t31691: f64, t31675: f64, t31681: f64, t31690: f64, t33115: f64, t8512: f64, t8515: f64) -> (f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t33560 = t12571 * t8511;
    let t33564 = t8513 * t8514 * t1437;
    let t33567 = t31682 * t1409;
    let t33568 = t8308 * t33567;
    let t33572 = t8513 * t31691 * t1433;
    let t33578 = piecewise3(t8, 0.0_f64, -5.0_f64 / 72.0_f64 * t33560 * t8515 + 5.0_f64 / 12.0_f64 * t31675 * t33564 + 5.0_f64 / 18.0_f64 * t31681 * t33568 + t31690 - 5.0_f64 / 36.0_f64 * t8512 * t33572 - 5.0_f64 / 72.0_f64 * t8512 * t33115);
    (t33560, t33564, t33568, t33572, t33578)
}
