//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 864/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk864<F: Float>(t12922: F, t12927: F, t12929: F, t12931: F, t12933: F, t12948: F, t12954: F, t12959: F, t12975: F, t12985: F, t12989: F, t1173: F) -> (F, F) {
    let t12992 = -t12975 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t12929 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t12933 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t12948 + t12931 / F::cast_from(3.0_f64) - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t12922 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t12954 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t12985 - F::cast_from(2.0_f64) * t12959 + F::cast_from(2.0_f64) * t12989 - t12927 / F::cast_from(3.0_f64);
    let t12993 = t1173 * t12992;
    (t12992, t12993)
}
