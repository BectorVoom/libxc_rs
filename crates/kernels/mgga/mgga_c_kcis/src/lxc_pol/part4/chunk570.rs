//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 570/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk570<F: Float>(t1079: F, t2850: F, t1056: F, t2829: F, t2845: F, t113: F, t2844: F, t3054: F, t331: F, t829: F, t160: F, t330: F) -> (F, F, F, F, F, F, F, F) {
    let t3136 = t1079 * t2850;
    let t3139 = t1056 * t2850;
    let t3142 = t1079 * t2829;
    let t3145 = t1056 * t2845;
    let t3150 = t113 * t2844;
    let t3153 = 0.23911438650126355246e-1 * t3054;
    let t3154 = t331 * t829;
    let t3158 = t160 * t330;
    (t3136, t3139, t3142, t3145, t3150, t3153, t3154, t3158)
}
