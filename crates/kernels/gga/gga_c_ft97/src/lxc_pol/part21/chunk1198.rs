//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1198/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1198<F: Float>(t29807: F, t8392: F, t102954: F, t102960: F, t102997: F, t102999: F, t103010: F, t103013: F, t11593: F, t11854: F, t16006: F, t16007: F, t16156: F, t16161: F, t16183: F, t16241: F, t1866: F, t1901: F, t23265: F, t23323: F, t23327: F, t29956: F, t379: F, t4454: F, t446: F, t5750: F, t8557: F) -> (F,) {
    let t117363 = t8392 * t29807;
    let t117387 = -t1901 * t8557 * t29956 * t379 / 9.0 + t102954 - t102960 - 2.0 / 9.0 * t1901 * t23323 * t16161 - t102997 - 2.0 / 81.0 * t117363 + t102999 + 4.0 / 9.0 * t11593 * t23323 * t16183 - 8.0 / 27.0 * t103010 + t103013 + t1901 * t23323 * t16007 / 9.0 - 2.0 / 9.0 * t1901 * t23327 * t16156 - 2.0 / 27.0 * t446 * t1866 * t5750 * t4454 - t1901 * t8557 * t23265 * t16241 / 9.0 - 2.0 / 9.0 * t1901 * t11854 * t23265 * t16006;
    (t117387,)
}
