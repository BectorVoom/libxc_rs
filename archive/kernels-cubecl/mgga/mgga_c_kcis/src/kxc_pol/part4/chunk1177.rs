//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1177/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1177<F: Float>(t1154: F, t167: F, t3405: F, t3393: F, t5139: F, t5147: F, t8931: F, t10594: F, t1646: F, t2943: F, t365: F, t2635: F, t5153: F) -> (F, F, F, F, F, F) {
    let t14922 = t1154 * t3405 * t167;
    let t14926 = F::cast_from(0.35374814814814814814e-1_f64) * t3393 * t5139;
    let t14927 = t8931 * t5147;
    let t14930 = t1154 * t10594 * t1646;
    let t14940 = t365 * t2943;
    let t14944 = t1154 * t5153 * t2635;
    (t14922, t14926, t14927, t14930, t14940, t14944)
}
