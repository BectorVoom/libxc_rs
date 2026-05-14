//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 948/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk948<F: Float>(t1339: F, t1866: F, t4454: F, t447: F, t6564: F, t925: F, t110: F, t29569: F, t452: F, t1901: F, t26139: F, t26192: F, t29799: F, t29803: F, t29807: F, t29810: F, t29813: F, t29817: F, t29824: F, t29828: F, t29833: F, t29836: F, t29841: F, t29845: F, t446: F) -> (F, F, F, F) {
    let t29849 = t1866 * t1339 * t4454;
    let t29853 = t447 * t6564 * t925;
    let t29857 = t452 * t110 * t29569;
    let t29860 = -4.0 / 3.0 * t1901 * t29799 + t1901 * t29803 / 9.0 + 2.0 / 27.0 * t1901 * t29807 - t446 * t29810 / 3.0 - 2.0 / 3.0 * t446 * t29813 - t446 * t29817 / 3.0 + 2.0 / 9.0 * t26139 + 2.0 / 9.0 * t26192 + t446 * t29824 / 3.0 + 2.0 / 3.0 * t446 * t29828 + 2.0 / 3.0 * t446 * t29833 - 2.0 * t446 * t29836 - 2.0 / 3.0 * t446 * t29841 - t446 * t29845 / 9.0 - 2.0 / 27.0 * t446 * t29849 - 2.0 / 9.0 * t446 * t29853 - t446 * t29857 / 3.0;
    (t29849, t29853, t29857, t29860)
}
