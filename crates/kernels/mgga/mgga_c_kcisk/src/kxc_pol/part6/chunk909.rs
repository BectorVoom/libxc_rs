//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 909/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk909<F: Float>(t7099: F, t8708: F, t23460: F, t23606: F, t23609: F, t29082: F, t29085: F, t29091: F, t29097: F, t29152: F, t29155: F, t29161: F, t29164: F, t29166: F, t29168: F) -> (F, F) {
    let t29170 = t7099 * t8708;
    let t29172 = -F::cast_from(0.33547222222222222222e0_f64) * t29082 + F::cast_from(0.12077e1_f64) * t29085 - F::cast_from(0.181155e1_f64) * t29091 - F::cast_from(0.301925e0_f64) * t29097 - F::cast_from(0.73586666666666666666e-1_f64) * t29152 - F::cast_from(0.16557e0_f64) * t29155 + F::cast_from(0.20128333333333333333e0_f64) * t23460 + F::cast_from(0.11038e0_f64) * t23606 + F::cast_from(0.33114e0_f64) * t23609 + F::cast_from(0.33114e0_f64) * t29161 - F::cast_from(0.99342e0_f64) * t29164 + F::cast_from(0.16504875e0_f64) * t29166 + F::cast_from(0.247573125e0_f64) * t29168 - F::cast_from(0.3883875e1_f64) * t29170;
    (t29170, t29172)
}
