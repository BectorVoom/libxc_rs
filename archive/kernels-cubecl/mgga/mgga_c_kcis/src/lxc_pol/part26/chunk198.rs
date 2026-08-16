//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 198/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk198<F: Float>(t174: F, t176: F, t833: F, t44: F, t832: F, t194: F, t189: F, t651: F, t653: F, t657: F, t659: F, t197: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t175 = t174 <= zeta_threshold;
    let t836 = piecewise3::<F>(t175, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t176 * t833);
    let t838 = (t832 + t836) * t44;
    let t843 = t194 * t194;
    let t844 = F::cast_from(1.0_f64) / t843;
    let t845 = t189 * t844;
    let t850 = -F::cast_from(0.1176575e1_f64) * t651 - F::cast_from(0.516475e0_f64) * t653 - F::cast_from(0.2103875e0_f64) * t657 - F::cast_from(0.104195e0_f64) * t659;
    let t851 = F::cast_from(1.0_f64) / t197;
    (t838, t843, t844, t845, t850, t851)
}
