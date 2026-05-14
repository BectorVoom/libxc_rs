//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 969/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk969<F: Float>(t1647: F, t3036: F, t3037: F, t15407: F, t1629: F, t3073: F, t4180: F, t4203: F, t377: F, t5307: F, t1160: F, t16548: F, t13584: F, t16171: F, t1004: F, t5304: F) -> (F, F, F, F, F, F, F) {
    let t19196 = t3036 * t1647 * t3037;
    let t19199 = t3073 * t1629 * t15407;
    let t19208 = t4180 * t4203;
    let t19213 = t377 * t5307;
    let t19216 = t1160 * t1629 * t16548;
    let t19222 = t13584 * t1629 * t16171;
    let t19224 = t1004 * t5304;
    (t19196, t19199, t19208, t19213, t19216, t19222, t19224)
}
