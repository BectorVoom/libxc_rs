//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 168/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk168<F: Float>(t656: F, t664: F, t579: F) -> (F, F, F, F) {
    let t667 = F::cast_from(1.0_f64) + F::cast_from(0.5397236614853195164e-1_f64) * t656 * t664;
    let t668 = F::ln(t667);
    let t670 = F::cast_from(1.0_f64) + F::cast_from(0.193e0_f64) * t668;
    let t671 = F::cast_from(1.0_f64) / t670;
    let t673 = F::cast_from(1.0_f64) / t579;
    (t667, t670, t671, t673)
}
