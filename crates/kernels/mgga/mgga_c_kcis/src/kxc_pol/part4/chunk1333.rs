//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1333/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1333<F: Float>(t1437: F, t16082: F, t1330: F, t16060: F, t16078: F, t16069: F, t16055: F, t3883: F, t16065: F, t5845: F, t743: F, t5848: F) -> (F, F, F, F, F, F, F, F) {
    let t17155 = t1437 * t16082;
    let t17158 = t1330 * t16060;
    let t17161 = t1437 * t16078;
    let t17164 = t1330 * t16069;
    let t17167 = t3883 * t16055;
    let t17170 = t1330 * t16065;
    let t17174 = F::new(0.4705225e-4) * t743 * t5845;
    let t17175 = t743 * t5848;
    (t17155, t17158, t17161, t17164, t17167, t17170, t17174, t17175)
}
