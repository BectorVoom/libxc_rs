//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1365/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1365<F: Float>(t26919: F, t46862: F, t12602: F, t12609: F, t12625: F, t12956: F, t13070: F, t13140: F, t1643: F, t1901: F, t2205: F, t2221: F, t23443: F, t23455: F, t23456: F, t23900: F, t23968: F, t26981: F, t26991: F, t26999: F, t27334: F, t27335: F, t3429: F, t3578: F, t446: F, t47659: F, t50268: F, t574: F, t5942: F, t63180: F, t6725: F, t9144: F, t925: F, t9419: F, t95548: F, t95559: F, t95822: F, t95842: F) -> (F,) {
    let t106496 = t46862 * t26919;
    let t106499 = t446 * t574 * t3578 * t23968 / 3.0 - 4.0 / 3.0 * t1901 * t63180 * t23456 - 4.0 / 3.0 * t1901 * t50268 * t26981 + t1901 * t2221 * t95822 * t925 / 9.0 + 2.0 / 9.0 * t1901 * t9419 * t26991 - 2.0 / 27.0 * t446 * t2205 * t6725 * t1643 - 4.0 / 3.0 * t1901 * t13140 * t23455 * t12956 - 4.0 * t1901 * t27334 * t27335 * t12602 + 2.0 * t1901 * t26999 * t5942 * t13070 + 2.0 / 9.0 * t95548 - 2.0 / 9.0 * t1901 * t23443 * t12625 + 4.0 / 9.0 * t47659 * t95842 * t12609 - 2.0 / 9.0 * t1901 * t9144 * t23900 * t3429 - 22.0 / 27.0 * t106496 - 2.0 / 9.0 * t95559;
    (t106499,)
}
