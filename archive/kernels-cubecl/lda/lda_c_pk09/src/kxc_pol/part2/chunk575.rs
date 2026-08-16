//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 575/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk575<F: Float>(t3194: F, t4064: F, t2974: F, t1062: F, t703: F, t721: F, t191: F, t2971: F, t1067: F, t773: F, t3743: F, t932: F) -> (F, F, F, F, F, F) {
    let t4065 = t4064 * t3194;
    let t4067 = t4064 * t2974;
    let t4069 = t703 * t1062;
    let t4070 = t4069 * t721;
    let t4072 = t191 * t2971;
    let t4077 = t773 * t1067;
    let t4085 = t932 * t3743;
    (t4065, t4067, t4070, t4072, t4077, t4085)
}
