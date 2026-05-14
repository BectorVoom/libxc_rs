//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1369/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1369<F: Float>(t158: F, t27335: F, t526: F, t9439: F, t104566: F, t104575: F, t11593: F, t12609: F, t12650: F, t12738: F, t12992: F, t12999: F, t13088: F, t13220: F, t13221: F, t144: F, t1901: F, t2185: F, t2190: F, t23470: F, t23571: F, t23581: F, t26523: F, t26999: F, t27000: F, t27414: F, t3450: F, t3590: F, t379: F, t446: F, t47659: F, t569: F, t574: F, t5860: F, t5935: F, t597: F, t6708: F, t9016: F, t95837: F) -> (F,) {
    let t106651 = t158 * t27335;
    let t106698 = t526 * t9439;
    let t106703 = 4.0 / 9.0 * t47659 * t95837 * t12609 + 4.0 / 3.0 * t47659 * t106651 * t13221 - 4.0 * t1901 * t9016 * t597 * t27000 - 4.0 * t1901 * t26999 * t23581 * t3450 + 2.0 * t1901 * t26999 * t23571 * t13088 - 4.0 / 9.0 * t1901 * t13220 * t26523 * t379 + 8.0 / 9.0 * t11593 * t23470 * t12999 + t1901 * t23470 * t12738 / 9.0 + 4.0 / 9.0 * t11593 * t23470 * t12992 + 4.0 / 3.0 * t446 * t144 * t104575 + 2.0 / 3.0 * t446 * t144 * t104566 + 2.0 / 3.0 * t446 * t574 * t5935 * t12650 + 4.0 / 3.0 * t446 * t2185 * t3590 * t5860 - 2.0 / 9.0 * t446 * t569 * t27414 * t379 + 4.0 * t1901 * t106698 * t6708 * t2190;
    (t106703,)
}
