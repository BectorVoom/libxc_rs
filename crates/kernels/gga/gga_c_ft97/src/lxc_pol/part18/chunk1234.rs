//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1234/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1234<F: Float>(t11960: F, t1332: F, t1852: F, t6540: F, t8232: F, t1882: F, t26337: F, t100226: F, t11468: F, t11593: F, t11623: F, t11832: F, t1647: F, t1901: F, t1902: F, t1909: F, t22940: F, t23244: F, t23270: F, t23327: F, t23339: F, t26042: F, t26171: F, t26198: F, t26202: F, t26267: F, t3052: F, t3214: F, t38711: F, t432: F, t446: F, t447: F, t452: F, t47089: F, t60711: F, t6564: F, t83: F, t91537: F) -> (F, F) {
    let t102610 = t1852 * t1332 * t11960;
    let t102614 = t8232 * t6540;
    let t102626 = 4.0 / 9.0 * t1882 * t26337;
    let t102654 = -2.0 / 3.0 * t446 * t452 * t26042 * t432 + 2.0 / 3.0 * t446 * t83 * t102610 + 4.0 / 27.0 * t102614 + 2.0 / 81.0 * t91537 + 2.0 / 9.0 * t446 * t447 * t6564 * t1647 + 2.0 / 3.0 * t446 * t452 * t22940 * t3214 - t102626 - 4.0 / 9.0 * t1901 * t60711 * t23270 + 4.0 / 9.0 * t11593 * t1902 * t23244 * t3052 + 2.0 * t1901 * t26171 * t23339 * t11623 - 2.0 / 9.0 * t1901 * t1909 * t26267 * t1647 - 2.0 / 9.0 * t1901 * t23327 * t11832 - 2.0 / 9.0 * t1901 * t38711 * t26198 - 4.0 / 9.0 * t1901 * t47089 * t26202 - 2.0 / 9.0 * t1901 * t11468 * t100226;
    (t102610, t102654)
}
