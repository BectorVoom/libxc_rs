//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 553/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk553<F: Float>(t3073: F, t4242: F, t1647: F, t864: F, t1035: F, t151: F, t3075: F, t3078: F, t3081: F, t3091: F, t3104: F, t3827: F, t3830: F, t4228: F, t4230: F, t4231: F, t4234: F, t4235: F, t4238: F) -> (F,) {
    let t4244 = 0.26341796731742046394e1 * t3073 * t4242;
    let t4245 = t1647 * t864;
    let t4246 = t1035 * t4245;
    let t4249 = -0.13170898365871023197e1 * t3075 + 0.26341796731742046395e1 * t3078 + 0.65854491829355115987e0 * t3081 - 0.26341796731742046394e1 * t3091 - t4228 - t4230 - 0.65854491829355115987e0 * t4231 - t4234 - t3104 + 0.65854491829355115987e0 * t4235 - 0.13170898365871023197e1 * t151 * t4238 - t4244 + 0.13170898365871023197e1 * t4246 - t3827 - 0.65854491829355115987e0 * t3830;
    (t4249,)
}
