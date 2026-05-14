//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1068/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1068<F: Float>(t2233: F, t29677: F, t12861: F, t1607: F, t4314: F, t4455: F, t779: F, t9274: F, t2531: F, t2537: F, t782: F, t9266: F, t142: F, t164: F, t9273: F, t113: F, t8750: F) -> (F, F, F, F, F, F, F, F) {
    let t29678 = t2233 * t29677;
    let t29679 = t29678 / 16.0;
    let t30409 = t1607 * t12861;
    let t30424 = t4455 * t4314;
    let t31271 = t779 * t9274;
    let t31274 = t2531 * t2537;
    let t35630 = t9266 * t782;
    let t35635 = t142 / t9273 / t164;
    let t36222 = t113 * t8750;
    (t29679, t30409, t30424, t31271, t31274, t35630, t35635, t36222)
}
