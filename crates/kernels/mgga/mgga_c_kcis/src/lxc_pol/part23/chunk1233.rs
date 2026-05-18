//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1233/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1233<F: Float>(t1394: F, t16995: F, t27387: F, t17287: F, t491: F, t990: F, t1385: F, t167: F, t16892: F, t27356: F, t16782: F, t16906: F, t27353: F, t27359: F, t27369: F, t27438: F, t28369: F, t28392: F, t7908: F, t98087: F, t98102: F, t98105: F, t98107: F) -> (F, F, F, F) {
    let t98110 = t1394 * t27387 * t16995;
    let t98119 = t17287 * t491 * t990;
    let t98124 = t16892 * t27356 * t167 * t1385;
    let t98131 = -F::new(0.12356481481481481481e-2) * t28392 * t27353 - F::new(0.33163888888888888888e-2) * t98102 - t98105 + F::new(0.33163888888888888888e-2) * t98107 + F::new(0.33163888888888888888e-2) * t98110 - F::new(0.46336805555555555556e-3) * t7908 * t98087 + F::new(0.46336805555555555556e-3) * t28369 * t27353 + F::new(0.46336805555555555556e-3) * t28369 * t27359 + F::new(0.61836467013888888889e-4) * t98119 * t27359 - F::new(0.12367293402777777778e-3) * t27369 * t98124 + F::new(0.12356481481481481482e-2) * t7908 * t16906 * t27438 * t16782;
    (t98110, t98119, t98124, t98131)
}
