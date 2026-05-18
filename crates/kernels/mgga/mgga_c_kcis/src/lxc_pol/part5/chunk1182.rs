//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1182/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1182<F: Float>(t18506: F, t19146: F, t19549: F, t19624: F, t19670: F, t19732: F, t19775: F, t19821: F, t393: F, t1141: F, t6634: F, t1203: F) -> (F, F) {
    let t19824 = t18506 + t19146 + t19549 + t19624 + t19670 + t19732 + t19775 + t19821;
    let t19825 = t19824 * t393;
    let t19826 = t6634 * t1141;
    let t19827 = t19826 * t1203;
    (t19825, t19827)
}
