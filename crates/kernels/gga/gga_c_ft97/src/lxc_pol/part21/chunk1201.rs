//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1201/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1201<F: Float>(t29726: F, t487: F, t492: F, t103082: F, t103083: F, t103107: F, t11552: F, t11593: F, t116097: F, t116127: F, t116157: F, t116316: F, t11906: F, t16061: F, t16066: F, t16165: F, t16178: F, t1825: F, t1901: F, t23244: F, t23323: F, t23327: F, t26268: F, t29831: F, t4454: F, t446: F, t452: F, t83: F, t8518: F) -> (F, F) {
    let t117485 = t29726 * t487;
    let t117486 = t117485 * t492;
    let t117520 = 8.0 / 27.0 * t11593 * t11552 * t116316 - 2.0 * t446 * t83 * t116127 - t446 * t83 * t117486 / 3.0 + 2.0 / 3.0 * t446 * t83 * t116157 + t103082 - 4.0 / 27.0 * t103083 + 2.0 / 3.0 * t446 * t452 * t1825 * t29831 + 4.0 / 3.0 * t446 * t83 * t116097 - 2.0 / 3.0 * t1901 * t23323 * t16165 + 2.0 / 9.0 * t1901 * t23323 * t16178 + 2.0 / 9.0 * t1901 * t11906 * t26268 + 4.0 / 9.0 * t11593 * t23327 * t16066 + 2.0 / 9.0 * t1901 * t23327 * t16061 - t103107 + 2.0 / 27.0 * t1901 * t8518 * t23244 * t4454;
    (t117486, t117520)
}
