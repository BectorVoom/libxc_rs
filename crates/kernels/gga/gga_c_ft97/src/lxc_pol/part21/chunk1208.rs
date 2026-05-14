//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1208/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1208<F: Float>(t1882: F, t29876: F, t103068: F, t103108: F, t103515: F, t110: F, t11490: F, t116110: F, t116145: F, t116608: F, t11810: F, t16053: F, t16203: F, t1871: F, t1901: F, t1909: F, t23294: F, t23327: F, t25996: F, t26372: F, t26373: F, t29569: F, t29798: F, t3214: F, t446: F, t4462: F, t452: F, t4572: F, t4589: F, t47120: F, t488: F, t492: F, t5617: F, t83: F, t91817: F, t986: F) -> (F,) {
    let t117789 = t1882 * t29876;
    let t117841 = -2.0 / 9.0 * t117789 + 4.0 / 3.0 * t446 * t83 * t116110 + t1901 * t1909 * t23294 * t4462 / 9.0 - 4.0 / 3.0 * t1901 * t47120 * t29798 - 4.0 / 3.0 * t1901 * t11490 * t91817 * t4572 - 4.0 / 3.0 * t1901 * t11490 * t103068 * t3214 + 4.0 / 3.0 * t446 * t83 * t116145 - 4.0 * t1901 * t26372 * t26373 * t16203 + t446 * t452 * t488 * t5617 * t4589 / 3.0 + 4.0 / 3.0 * t446 * t1871 * t986 * t25996 + t1901 * t23327 * t16053 / 9.0 + 2.0 / 3.0 * t446 * t1871 * t110 * t116608 + t446 * t452 * t488 * t29569 * t492 / 3.0 + t103515 - 4.0 / 3.0 * t1901 * t11810 * t103108 * t3214;
    (t117841,)
}
