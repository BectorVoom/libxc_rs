//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1027/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1027<F: Float>(t330: F, t4920: F, t3393: F, t5139: F, t5147: F, t8931: F, t2943: F, t365: F, t11: F, t41: F, t85: F, t5143: F) -> (F, F, F, F, F, F) {
    let t14915 = t4920 * t330;
    let t14926 = F::cast_from(0.35374814814814814814e-1_f64) * t3393 * t5139;
    let t14927 = t8931 * t5147;
    let t14940 = t365 * t2943;
    let t14954 = t11 * t41;
    let t14955 = t85 * t14954;
    let t14956 = t14955 * t5143;
    (t14915, t14926, t14927, t14940, t14955, t14956)
}
