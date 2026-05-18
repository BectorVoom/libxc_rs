//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1044/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1044<F: Float>(t1198: F, t1350: F, t384: F, t398: F, t4552: F, t997: F, t12572: F, t4488: F, t1140: F, t5171: F, t1315: F, t13787: F) -> (F, F, F, F, F) {
    let t18066 = t384 * t398 * t1198 * t1350;
    let t18072 = t997 * t4552;
    let t18079 = t12572 * t4488;
    let t18085 = t1140 * t5171;
    let t18087 = t13787 * t1315;
    (t18066, t18072, t18079, t18085, t18087)
}
