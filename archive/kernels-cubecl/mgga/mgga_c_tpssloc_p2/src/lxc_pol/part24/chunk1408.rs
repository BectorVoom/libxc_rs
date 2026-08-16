//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1408/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1408<F: Float>(t265: F, t394: F, t1068: F, t1070: F, t11087: F, t11091: F, t193: F, t23738: F, t23742: F, t3209: F, t3213: F, t336: F, t4700: F, t50775: F, t6822: F, t82389: F, t82439: F, t82492: F, t83307: F, t83341: F, t83376: F, t83417: F, t83461: F, t83468: F, t83472: F, t83479: F, t83543: F) -> F {
    let t395 = t265 < t394;
    let t83544 = piecewise3::<F>(t395, t193 * t336 * (t82389 + t82439 + t82492 + t83307 + t83341 + t83376 + t83417 + t83461) * t1070 - F::cast_from(3.0_f64) * t4700 * t83468 * t1068 + F::cast_from(6.0_f64) * t4700 * t83472 * t3213 - F::cast_from(3.0_f64) * t4700 * t23738 * t3209 - F::cast_from(6.0_f64) * t4700 * t83479 * t11091 + F::cast_from(6.0_f64) * t4700 * t23742 * t50775 - t4700 * t6822 * t11087, t83543);
    t83544
}
