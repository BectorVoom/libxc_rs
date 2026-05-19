//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 752/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk752<F: Float>(t174: F, t4518: F, t4521: F, t740: F, t833: F, t44: F, t4517: F, t230: F, t1655: F, t908: F, t1659: F, t911: F, t2633: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t175 = t174 <= zeta_threshold;
    let t4525 = piecewise3::<F>(t175, F::new(0.0), F::new(4.0) / F::new(9.0) * t4518 * t833 - F::new(8.0) / F::new(3.0) * t4521 * t740);
    let t4527 = (t4517 + t4525) * t44;
    let t4528 = t4527 * t230;
    let t4529 = t1655 * t908;
    let t4530 = t911 * t1659;
    let t4532 = F::new(2.0) * t2633;
    (t4527, t4528, t4529, t4530, t4532)
}
