//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1360/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1360<F: Float>(t12599: F, t23997: F, t1384: F, t49553: F, t2179: F, t3565: F, t5968: F, t1349: F, t1637: F, t6588: F, t23938: F, t3578: F, t27253: F, t8392: F, t7763: F, t11437: F, t11593: F, t11604: F, t11982: F, t12709: F, t12714: F, t12986: F, t12991: F, t13220: F, t1647: F, t1901: F, t23548: F, t26836: F, t26935: F, t27215: F, t27216: F, t27220: F, t27221: F, t379: F, t40945: F, t49583: F, t50287: F, t50554: F, t6639: F, t6708: F, t9144: F) -> (F, F, F, F, F, F) {
    let t106182 = t23997 * t12599;
    let t106189 = t49553 * t1384;
    let t106197 = t2179 * t5968 * t3565;
    let t106200 = t1349 * t1637 * t6588;
    let t106204 = t3578 * t23938;
    let t106214 = 4.0 / 27.0 * t8392 * t27253;
    let t106253 = t1384 * t7763;
    let t106262 = -2.0 / 9.0 * t1901 * t40945 * t26935 + t106214 + 4.0 / 9.0 * t1901 * t13220 * t6708 * t1647 - 4.0 / 9.0 * t1901 * t50287 * t27216 + 4.0 / 27.0 * t1901 * t50554 * t27221 - 2.0 / 9.0 * t1901 * t9144 * t26836 * t379 - 4.0 / 9.0 * t11593 * t9144 * t23548 * t12986 - 8.0 / 9.0 * t11593 * t13220 * t23548 * t12991 - 8.0 / 9.0 * t11593 * t12709 * t27215 * t11604 + 8.0 / 27.0 * t11593 * t12714 * t27220 * t11604 + 2.0 / 9.0 * t1901 * t9144 * t6639 * t1647 + 2.0 / 27.0 * t1901 * t12714 * t27220 * t11982 + 10.0 / 81.0 * t1901 * t49583 * t106253 * t11437 - 2.0 / 9.0 * t1901 * t12709 * t27215 * t11982;
    (t106182, t106189, t106197, t106200, t106204, t106262)
}
