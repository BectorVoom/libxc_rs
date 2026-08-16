//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1977/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1977<F: Float>(t225: F, t29071: F, t1510: F, t24269: F, t2617: F, t26598: F, t28997: F, t29052: F, t4166: F, t5612: F, t5617: F, t812: F, t87068: F, t92491: F, t92546: F, t98325: F, t98328: F, t98330: F, t98334: F, t98339: F, t98342: F, t98345: F, t98349: F, t98353: F) -> (F, F) {
    let t101593 = t29071 * t225;
    let t101618 = F::cast_from(0.6579736267392905746e-1_f64) * t98325 - F::cast_from(2.0_f64) * t4166 * t26598 - F::cast_from(0.19739208802178717238e0_f64) * t98328 - F::cast_from(0.23029076935875170111e0_f64) * t98330 - F::cast_from(2.0_f64) * t812 * t92546 * t1510 - t812 * t24269 * t5617 + F::cast_from(0.16449340668482264365e-1_f64) * t98334 - t812 * t24269 * t5612 - F::cast_from(0.9869604401089358619e-1_f64) * t98339 - t2617 * t29052 + t92491 - F::cast_from(0.10417915756705434098e0_f64) * t87068 - F::cast_from(2.0_f64) * t2617 * t28997 - F::cast_from(0.82246703342411321825e-2_f64) * t98342 + F::cast_from(0.3289868133696452873e-1_f64) * t98345 - F::cast_from(0.3289868133696452873e-1_f64) * t98349 - F::cast_from(0.3289868133696452873e-1_f64) * t98353;
    (t101593, t101618)
}
