//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 822/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk822<F: Float>(t1786: F, t1825: F, t3281: F, t494: F, t454: F, t1822: F, t8232: F, t1893: F, t110: F, t1866: F, t1901: F, t38079: F, t38103: F, t38254: F, t432: F, t446: F, t447: F, t452: F, t488: F, t492: F, t8183: F, t8549: F, t8558: F, t8590: F) -> (F,) {
    let t38711 = t1786 * t1825;
    let t38732 = t3281 * t494;
    let t38734 = t3281 * t454;
    let t38740 = t8232 * t1822;
    let t38742 = t8232 * t1893;
    let t38744 = 4.0 * t446 * t452 * t1825 * t8549 - 8.0 / 3.0 * t1901 * t38711 * t8558 - 2.0 / 9.0 * t446 * t1866 * t110 * t38079 + 2.0 / 3.0 * t446 * t447 * t110 * t38103 + 4.0 / 3.0 * t446 * t452 * t488 * t8183 * t492 - t446 * t452 * t110 * t38254 / 3.0 + 112.0 / 81.0 * t38732 + 112.0 / 81.0 * t38734 - 4.0 / 3.0 * t446 * t452 * t8590 * t432 - 8.0 / 9.0 * t38740 - 16.0 / 9.0 * t38742;
    (t38744,)
}
