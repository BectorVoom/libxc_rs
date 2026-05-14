//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1060/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1060<F: Float>(t2825: F, t6508: F, t2842: F, t19159: F, t4546: F, t3202: F, t3200: F, t6696: F, t922: F, t1121: F, t6613: F, t1022: F, t3227: F, t1092: F, t6504: F, t1020: F) -> (F, F, F, F, F, F) {
    let t19778 = t2825 * t6508;
    let t19779 = t2842 * t19778;
    let t19781 = t4546 * t19159;
    let t19782 = t3202 * t19781;
    let t19783 = t3200 * t19782;
    let t19785 = t6696 * t922;
    let t19786 = t3202 * t19785;
    let t19787 = t3200 * t19786;
    let t19789 = t6613 * t1121;
    let t19790 = t1022 * t19789;
    let t19791 = t3227 * t19790;
    let t19792 = t1092 * t19791;
    let t19799 = t2825 * t6504;
    let t19800 = t1020 * t19799;
    (t19779, t19783, t19787, t19789, t19792, t19800)
}
