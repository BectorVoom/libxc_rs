//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1082/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1082<F: Float>(t1526: F, t42262: F, t5198: F, t1095: F, t13411: F, t17818: F, t18089: F, t226: F, t3773: F, t5049: F, t709: F, t66422: F, t688: F, t52358: F, t5025: F, t1127: F, t3817: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t72992 = t1526 * t42262 * t5198;
    let t79252 = t13411 * t1095;
    let t79253 = t79252 * t17818;
    let t79402 = t18089 * t226;
    let t79403 = t79402 * t3773;
    let t79485 = t5049 * t709;
    let t79528 = t66422 * t688;
    let t79529 = t79528 * t17818;
    let t79542 = t52358 * t688;
    let t79601 = t5025 * t709;
    let t79605 = t1127 * t3817;
    (t72992, t79252, t79253, t79402, t79403, t79485, t79528, t79529, t79542, t79601, t79605)
}
