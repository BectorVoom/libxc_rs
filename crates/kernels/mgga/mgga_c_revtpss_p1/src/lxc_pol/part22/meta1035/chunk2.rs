//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3622/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3622<F: Float>(t1189: F, t1196: F, t20382: F, t3495: F, t20472: F, t3498: F, t198: F, t336: F, t3801: F, t68243: F, t68245: F, t68247: F, t68250: F, t68602: F, t68604: F, t68608: F, t68611: F, t68613: F, t68614: F, t68621: F) -> (F, F, F) {
    let t68625 = F::cast_from(0.23392894490538584828e1_f64) * t1196 * t3495 * t20382 * t1189;
    let t68628 = F::cast_from(0.14035736694323150897e2_f64) * t1196 * t20472 * t3498;
    let t68629 = -F::new(2.0) * t198 * t336 * t3801 * t68614 - t68243 - t68245 - t68247 - t68250 - t68602 - t68604 - t68608 - t68611 - t68613 + t68621 + t68625 + t68628;
    (t68625, t68628, t68629)
}
