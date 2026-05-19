//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 860/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk860<F: Float>(t3101: F, t322: F, t317: F, t863: F, t3054: F, t441: F, t865: F, t3912: F, t868: F, t847: F, t861: F, t180: F) -> (F, F, F, F, F, F) {
    let t12254 = t322 * t3101;
    let t12257 = F::cast_from(0.52683593463484092788e1_f64) * t863 * t317 * t12254;
    let t12259 = t3054 * t441 * t865;
    let t12263 = t868 * t3912;
    let t12265 = t847 * t861;
    let t12268 = F::cast_from(0.79025390195226139183e1_f64) * t12265 * t180 * t865;
    (t12254, t12257, t12259, t12263, t12265, t12268)
}
