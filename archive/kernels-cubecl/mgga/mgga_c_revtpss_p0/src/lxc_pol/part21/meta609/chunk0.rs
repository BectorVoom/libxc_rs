//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2351/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2351<F: Float>(t2548: F, t2490: F, t2595: F, t39490: F, t39492: F, t39495: F, t39498: F, t39501: F, t39506: F, t39508: F, t39510: F, t39512: F, t39515: F) -> (F, F, F) {
    let t39886 = t2548 * t2548;
    let t39894 = F::cast_from(1.0_f64) / t2490 / t2595;
    let t39909 = -F::cast_from(0.28769444444444444444e1_f64) * t39490 + F::cast_from(0.27618666666666666667e2_f64) * t39492 - F::cast_from(0.10229135802469135803e2_f64) * t39495 + F::cast_from(0.89504938271604938273e1_f64) * t39498 + F::cast_from(0.31310740740740740741e1_f64) * t39501 + F::cast_from(0.366775e-1_f64) * t39506 - F::cast_from(0.58684e0_f64) * t39508 + F::cast_from(0.65204444444444444445e0_f64) * t39510 + F::cast_from(0.5705388888888888889e0_f64) * t39512 + F::cast_from(0.13490888888888888889e1_f64) * t39515;
    (t39886, t39894, t39909)
}
