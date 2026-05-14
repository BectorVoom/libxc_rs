//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 852/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk852<F: Float>(t18506: F, t19146: F, t19549: F, t19624: F, t19670: F, t19732: F, t19775: F, t19821: F, t393: F, t1141: F, t6634: F, t1203: F, t14665: F, t1820: F, t14668: F, t5039: F) -> (F, F, F, F, F, F) {
    let t19824 = t18506 + t19146 + t19549 + t19624 + t19670 + t19732 + t19775 + t19821;
    let t19825 = t19824 * t393;
    let t19826 = t6634 * t1141;
    let t19827 = t19826 * t1203;
    let t19829 = 2.0 * t14665 * t1820;
    let t19831 = 4.0 * t14668 * t5039;
    (t19824, t19825, t19826, t19827, t19829, t19831)
}
