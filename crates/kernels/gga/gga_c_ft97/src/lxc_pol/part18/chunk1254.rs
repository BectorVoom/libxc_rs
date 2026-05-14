//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1254/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1254<F: Float>(t23339: F, t47667: F, t1882: F, t26340: F, t26276: F, t100349: F, t102420: F, t11490: F, t11557: F, t11810: F, t1852: F, t1853: F, t1901: F, t23294: F, t26154: F, t3205: F, t3266: F, t3271: F, t379: F, t446: F, t452: F, t46874: F, t47666: F, t6454: F, t83: F, t8557: F, t91817: F, t92006: F, t92014: F, t92021: F, t92024: F, t92025: F, t92049: F) -> (F, F) {
    let t103510 = t47667 * t23339;
    let t103515 = 2.0 / 9.0 * t1882 * t26340;
    let t103542 = 2.0 / 9.0 * t1882 * t26276;
    let t103548 = -4.0 / 27.0 * t47666 * t103510 * t11557 + t103515 + 4.0 / 3.0 * t446 * t83 * t102420 - 2.0 / 3.0 * t446 * t452 * t1852 * t6454 * t1853 - 2.0 / 81.0 * t92006 + 2.0 / 3.0 * t1901 * t46874 * t100349 - 4.0 / 9.0 * t92014 - 4.0 / 3.0 * t1901 * t11810 * t23294 * t3266 - t92021 / 27.0 - 4.0 / 3.0 * t1901 * t11490 * t91817 * t3271 + 2.0 / 9.0 * t1901 * t92049 * t3205 + t103542 - 2.0 / 9.0 * t1901 * t8557 * t26154 * t379 + t92024 + t92025 / 9.0;
    (t103510, t103548)
}
