//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1036/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1036<F: Float>(t174: F, t13014: F, t1650: F, t167: F, t2641: F, t160: F, t176: F, t2642: F, t2645: F, t4518: F, t4521: F, t740: F, t833: F, zeta_threshold: F) -> F {
    let t175 = t174 <= zeta_threshold;
    let t13077 = t13014 * t1650;
    let t13080 = t2641 * t167;
    let t13091 = piecewise3::<F>(t175, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t13077 * t2642 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t13080 * t740 * t833 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t4518 * t2645 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t176 * t740 + F::cast_from(8.0_f64) * t4521 * t160);
    t13091
}
