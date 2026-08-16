//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1408/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1408(t265: f64, t394: f64, t1068: f64, t1070: f64, t11087: f64, t11091: f64, t193: f64, t23738: f64, t23742: f64, t3209: f64, t3213: f64, t336: f64, t4700: f64, t50775: f64, t6822: f64, t82389: f64, t82439: f64, t82492: f64, t83307: f64, t83341: f64, t83376: f64, t83417: f64, t83461: f64, t83468: f64, t83472: f64, t83479: f64, t83543: f64) -> f64 {
    let t395 = t265 < t394;
    let t83544 = piecewise3(t395, t193 * t336 * (t82389 + t82439 + t82492 + t83307 + t83341 + t83376 + t83417 + t83461) * t1070 - 3.0_f64 * t4700 * t83468 * t1068 + 6.0_f64 * t4700 * t83472 * t3213 - 3.0_f64 * t4700 * t23738 * t3209 - 6.0_f64 * t4700 * t83479 * t11091 + 6.0_f64 * t4700 * t23742 * t50775 - t4700 * t6822 * t11087, t83543);
    t83544
}
