//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 906/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk906<F: Float>(t2408: F, t8701: F, t11036: F, t11056: F, t11040: F, t17382: F, t23460: F, t23472: F, t23481: F, t29082: F, t29085: F, t29088: F, t29091: F, t29094: F, t29097: F) -> (F, F, F) {
    let t29123 = t8701 * t2408;
    let t29124 = t11036 * t29123;
    let t29126 = t11056 * t29123;
    let t29138 = -t11040 - F::new(4.0) / F::new(9.0) * t17382 + F::new(2.0) / F::new(9.0) * t23460 - F::new(2.0) / F::new(3.0) * t23472 + t23481 / F::new(3.0) - F::new(10.0) / F::new(27.0) * t29082 + F::new(4.0) / F::new(3.0) * t29085 - F::new(2.0) / F::new(3.0) * t29088 - F::new(2.0) * t29091 + F::new(2.0) * t29094 - t29097 / F::new(3.0);
    (t29124, t29126, t29138)
}
