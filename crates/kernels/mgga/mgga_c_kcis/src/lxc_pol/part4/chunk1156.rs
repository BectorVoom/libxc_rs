//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1156/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1156<F: Float>(t14663: F, t393: F, t1141: F, t5034: F, t1203: F, t1778: F, t3329: F, t3331: F, t3481: F, t5036: F, t10488: F, t1820: F) -> (F, F, F, F, F) {
    let t14664 = t14663 * t393;
    let t14665 = t5034 * t1141;
    let t14667 = F::cast_from(2.0_f64) * t14665 * t1203;
    let t14668 = t1778 * t3329;
    let t14670 = F::cast_from(2.0_f64) * t14668 * t3331;
    let t14671 = t5036 * t3481;
    let t14672 = t10488 * t1820;
    (t14664, t14667, t14670, t14671, t14672)
}
