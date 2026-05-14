//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1255/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1255<F: Float>(t1882: F, t26242: F, t26259: F, t26159: F, t8392: F, t6549: F, t8232: F, t26402: F, t11520: F, t11613: F, t11828: F, t1339: F, t1643: F, t1825: F, t1866: F, t1871: F, t1901: F, t22943: F, t25846: F, t26154: F, t26378: F, t26382: F, t3266: F, t3291: F, t446: F, t452: F, t47007: F, t47120: F, t488: F, t492: F, t5617: F, t5750: F, t6564: F, t92055: F) -> (F,) {
    let t103550 = 2.0 / 9.0 * t1882 * t26242;
    let t103556 = 4.0 / 9.0 * t1882 * t26259;
    let t103571 = 2.0 / 27.0 * t8392 * t26159;
    let t103572 = t8232 * t6549;
    let t103592 = 2.0 / 9.0 * t1882 * t26402;
    let t103597 = -t103550 + 4.0 / 3.0 * t446 * t1871 * t5750 * t3266 - t103556 + 4.0 / 3.0 * t446 * t1871 * t1339 * t11520 - 2.0 / 9.0 * t1901 * t92055 * t11828 - 4.0 / 3.0 * t1901 * t47007 * t26378 - 4.0 / 3.0 * t1901 * t47120 * t26382 - t103571 + 8.0 / 27.0 * t103572 + 2.0 / 3.0 * t446 * t452 * t1825 * t26154 + 2.0 / 3.0 * t446 * t452 * t488 * t25846 * t492 - 2.0 / 27.0 * t446 * t1866 * t6564 * t1643 - 2.0 / 3.0 * t446 * t452 * t22943 * t11613 - t103592 - 2.0 / 3.0 * t446 * t452 * t3291 * t5617;
    (t103597,)
}
