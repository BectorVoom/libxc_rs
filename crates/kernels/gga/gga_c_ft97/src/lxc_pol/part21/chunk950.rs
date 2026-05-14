//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 950/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk950<F: Float>(t29792: F, t83: F, t29794: F, t1901: F, t23152: F, t26220: F, t26222: F, t26252: F, t26265: F, t26291: F, t26293: F, t29862: F, t29865: F, t29869: F, t29872: F, t29876: F, t29879: F, t29882: F, t29888: F, t446: F) -> (F, F, F) {
    let t29893 = t83 * t29792;
    let t29896 = t83 * t29794;
    let t29901 = -2.0 / 3.0 * t446 * t29862 + 2.0 / 3.0 * t446 * t29865 + 2.0 / 3.0 * t446 * t29869 + t23152 + 2.0 / 9.0 * t1901 * t29872 + 2.0 / 3.0 * t446 * t29876 + 2.0 / 3.0 * t446 * t29879 + 4.0 / 3.0 * t446 * t29882 + 2.0 / 9.0 * t26220 - 2.0 / 9.0 * t26222 + 2.0 / 9.0 * t446 * t29888 - 4.0 / 9.0 * t26252 - 2.0 / 27.0 * t26265 - 2.0 / 3.0 * t446 * t29893 - t446 * t29896 / 3.0 + 2.0 / 9.0 * t26291 + 2.0 / 9.0 * t26293;
    (t29893, t29896, t29901)
}
