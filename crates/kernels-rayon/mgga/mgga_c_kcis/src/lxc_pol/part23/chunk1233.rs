//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1233/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1233(t1394: f64, t16995: f64, t27387: f64, t17287: f64, t491: f64, t990: f64, t1385: f64, t167: f64, t16892: f64, t27356: f64, t16782: f64, t16906: f64, t27353: f64, t27359: f64, t27369: f64, t27438: f64, t28369: f64, t28392: f64, t7908: f64, t98087: f64, t98102: f64, t98105: f64, t98107: f64) -> (f64, f64, f64, f64) {
    let t98110 = t1394 * t27387 * t16995;
    let t98119 = t17287 * t491 * t990;
    let t98124 = t16892 * t27356 * t167 * t1385;
    let t98131 = -0.12356481481481481481e-2_f64 * t28392 * t27353 - 0.33163888888888888888e-2_f64 * t98102 - t98105 + 0.33163888888888888888e-2_f64 * t98107 + 0.33163888888888888888e-2_f64 * t98110 - 0.46336805555555555556e-3_f64 * t7908 * t98087 + 0.46336805555555555556e-3_f64 * t28369 * t27353 + 0.46336805555555555556e-3_f64 * t28369 * t27359 + 0.61836467013888888889e-4_f64 * t98119 * t27359 - 0.12367293402777777778e-3_f64 * t27369 * t98124 + 0.12356481481481481482e-2_f64 * t7908 * t16906 * t27438 * t16782;
    (t98110, t98119, t98124, t98131)
}
