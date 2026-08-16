//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 701/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk701<F: Float>(t13433: F, t1445: F, t4527: F, t11408: F, t874: F, t1562: F, t3377: F, t3566: F, t11362: F, t13296: F, t189: F, t188: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13434 = t1445 * t13433;
    let t13436 = F::cast_from(0.27606906686822939767e2_f64) * t4527 * t13434;
    let t13437 = t11408 * t874;
    let t13438 = t1445 * t13437;
    let t13440 = F::cast_from(0.69017266717057349418e1_f64) * t1562 * t13438;
    let t13442 = F::cast_from(0.25025342966295298669e1_f64) * t3566 * t3377;
    let t13444 = F::cast_from(0.10725146985555128001e1_f64) * t11362 * t3377;
    let t13445 = t189 * t13296;
    let t13446 = t188 * t13445;
    (t13434, t13436, t13437, t13438, t13440, t13442, t13444, t13445, t13446)
}
