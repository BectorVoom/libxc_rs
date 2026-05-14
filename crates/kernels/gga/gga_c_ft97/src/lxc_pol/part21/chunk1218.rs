//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1218/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1218<F: Float>(t103840: F, t103855: F, t11810: F, t1339: F, t16115: F, t16120: F, t16261: F, t1871: F, t1901: F, t26042: F, t26267: F, t29599: F, t3052: F, t3266: F, t3281: F, t379: F, t4436: F, t446: F, t4462: F, t447: F, t4495: F, t452: F, t4623: F, t47548: F, t5617: F, t5710: F, t5750: F, t6564: F, t925: F, t93676: F, t93677: F) -> (F,) {
    let t118253 = -4.0 / 3.0 * t1901 * t11810 * t26267 * t3266 + t93676 - t446 * t452 * t4623 * t5617 / 3.0 - 4.0 / 27.0 * t93677 - t446 * t452 * t5750 * t4495 / 3.0 - t103840 + 2.0 / 3.0 * t1901 * t47548 * t29599 * t379 + 2.0 / 3.0 * t446 * t1871 * t5750 * t4436 - 2.0 / 9.0 * t446 * t447 * t26042 * t925 + 2.0 / 3.0 * t446 * t1871 * t1339 * t16261 - 4.0 / 9.0 * t3281 * t447 * t6564 * t3052 - 2.0 / 3.0 * t446 * t1871 * t5710 * t16115 + 4.0 / 3.0 * t446 * t1871 * t1339 * t16120 - t446 * t447 * t5750 * t4462 / 9.0 + t103855;
    (t118253,)
}
