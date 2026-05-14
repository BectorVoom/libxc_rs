//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1207/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1207<F: Float>(t1882: F, t29988: F, t1851: F, t6454: F, t103453: F, t103459: F, t103486: F, t103488: F, t103490: F, t103626: F, t11490: F, t11810: F, t1339: F, t16110: F, t16286: F, t16291: F, t1825: F, t1901: F, t23339: F, t26166: F, t26372: F, t29822: F, t3214: F, t3219: F, t4458: F, t446: F, t447: F, t452: F, t5750: F, t60426: F, t6538: F, t6557: F, t8411: F, t8417: F) -> (F,) {
    let t117773 = t1882 * t29988;
    let t117775 = t1851 * t6454;
    let t117788 = t446 * t452 * t1825 * t29822 / 3.0 - t103453 + t103459 - 2.0 * t446 * t8411 * t1339 * t16110 + 8.0 / 3.0 * t1901 * t60426 * t6538 * t3214 + 4.0 * t1901 * t103626 * t6538 * t3219 - 4.0 * t1901 * t26372 * t8417 * t6557 * t3219 + 2.0 / 9.0 * t446 * t447 * t5750 * t4458 + 2.0 / 9.0 * t117773 - t103486 - t103488 - 4.0 / 3.0 * t1901 * t11490 * t117775 * t3219 - 2.0 / 3.0 * t1901 * t11810 * t23339 * t16286 - 2.0 / 3.0 * t1901 * t11490 * t26166 * t16291 - t103490;
    (t117788,)
}
