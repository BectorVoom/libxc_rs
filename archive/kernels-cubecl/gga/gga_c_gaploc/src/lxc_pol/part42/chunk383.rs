//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 383/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk383<F: Float>(t1445: F, t3410: F, t1562: F, t1024: F, t954: F, t2508: F, t3216: F, t3226: F, t3218: F, t3223: F, t471: F, t1020: F, t871: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3411 = t1445 * t3410;
    let t3413 = F::cast_from(0.69017266717057349418e1_f64) * t1562 * t3411;
    let t3420 = t954 * t1024;
    let t3422 = F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t3420;
    let t3423 = F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t3216;
    let t3426 = t3226 / F::cast_from(128.0_f64);
    let t3427 = t3423 - F::cast_from(9.0_f64) / F::cast_from(4096.0_f64) * t3218 + F::cast_from(3.0_f64) / F::cast_from(4096.0_f64) * t3223 - t3426;
    let t3428 = t3427 * t471;
    let t3429 = t1020 * t871;
    (t3411, t3413, t3420, t3422, t3423, t3426, t3427, t3428, t3429)
}
