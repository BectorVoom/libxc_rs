//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1199/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1199<F: Float>(t29944: F, t8392: F, t102783: F, t102948: F, t11472: F, t11902: F, t16011: F, t16110: F, t1901: F, t1902: F, t23244: F, t26184: F, t26198: F, t26202: F, t26435: F, t26436: F, t26441: F, t30008: F, t3255: F, t38711: F, t446: F, t4462: F, t452: F, t4606: F, t46809: F, t47443: F, t488: F, t5630: F, t59659: F, t59663: F, t60711: F, t6454: F, t6465: F, t8557: F, t91705: F, t91718: F) -> (F,) {
    let t117398 = t8392 * t29944;
    let t117435 = -2.0 / 9.0 * t1901 * t11472 * t26435 * t16011 + 2.0 / 3.0 * t446 * t452 * t488 * t6454 * t3255 + 2.0 / 27.0 * t117398 - 2.0 / 9.0 * t1901 * t38711 * t30008 + t1901 * t1902 * t23244 * t4462 / 9.0 - 2.0 / 9.0 * t1901 * t8557 * t102783 * t4606 + 8.0 * t1901 * t102948 * t5630 * t16110 + 8.0 / 27.0 * t91705 + 4.0 / 27.0 * t91718 - 2.0 / 9.0 * t1901 * t47443 * t26198 - 4.0 / 9.0 * t1901 * t60711 * t26202 - 4.0 / 9.0 * t1901 * t59659 * t26436 + 4.0 / 27.0 * t1901 * t59663 * t26441 + 2.0 / 9.0 * t1901 * t11902 * t26184 + 2.0 / 9.0 * t1901 * t46809 * t6465;
    (t117435,)
}
