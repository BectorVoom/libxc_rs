//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 681/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk681<F: Float>(t1683: F, t4761: F, t5412: F, t4730: F, t827: F, t10488: F, t4726: F, t26: F, t10442: F, t1659: F, t1660: F, t2877: F) -> (F, F, F, F, F) {
    let t10603 = t4761 * t1683;
    let t10604 = t10603 * t5412;
    let t10607 = t827 * t4730;
    let t10609 = t4726 * t10488;
    let t10610 = t26 * t10609;
    let t10612 = t1659 * t10442;
    let t10613 = t26 * t10612;
    let t10615 = t2877 * t1660;
    (t10604, t10607, t10610, t10613, t10615)
}
