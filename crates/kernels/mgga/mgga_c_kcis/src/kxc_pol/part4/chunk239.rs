//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 239/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk239<F: Float>(t174: F, t176: F, t833: F, t44: F, t832: F, t194: F, t189: F, t651: F, t653: F, t657: F, t659: F, t197: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t175 = t174 <= zeta_threshold;
    let t836 = piecewise3::<f64>(t175, F::new(0.0), F::new(4.0) / F::new(3.0) * t176 * t833);
    let t838 = (t832 + t836) * t44;
    let t843 = t194 * t194;
    let t844 = F::new(1.0) / t843;
    let t845 = t189 * t844;
    let t850 = -F::new(0.1176575e1) * t651 - F::new(0.516475e0) * t653 - F::new(0.2103875e0) * t657 - F::new(0.104195e0) * t659;
    let t851 = F::new(1.0) / t197;
    (t838, t843, t844, t845, t850, t851)
}
