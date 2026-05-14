//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1308/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1308<F: Float>(t30364: F, t8392: F, t30417: F, t1060: F, t106708: F, t106745: F, t106747: F, t106759: F, t11593: F, t12968: F, t16946: F, t17118: F, t17388: F, t1901: F, t2142: F, t2185: F, t2210: F, t23470: F, t23571: F, t26590: F, t26888: F, t27020: F, t30404: F, t30518: F, t3052: F, t3478: F, t446: F, t51036: F, t574: F, t5935: F, t6626: F, t9099: F) -> (F,) {
    let t120759 = t8392 * t30364;
    let t120777 = t8392 * t30417;
    let t120786 = 8.0 / 9.0 * t11593 * t23470 * t16946 + 4.0 / 9.0 * t11593 * t2210 * t27020 * t3052 + t1901 * t9099 * t30404 / 9.0 - t106708 + 2.0 / 27.0 * t120759 + 2.0 / 3.0 * t446 * t574 * t26590 * t3478 + 4.0 / 3.0 * t446 * t2185 * t1060 * t26888 + t446 * t574 * t5935 * t17388 / 3.0 - 2.0 / 3.0 * t446 * t2185 * t2142 * t30518 + 4.0 / 9.0 * t120777 + t106745 + t106747 + 2.0 / 9.0 * t1901 * t51036 * t6626 - t106759 - 4.0 / 3.0 * t1901 * t12968 * t23571 * t17118;
    (t120786,)
}
