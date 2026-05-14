//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 858/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk858<F: Float>(t358: F, t363: F, t9017: F, t39749: F, t446: F, t1647: F, t2075: F, t1969: F, t1643: F, t9049: F, t39708: F, t39711: F, t39715: F, t39717: F, t39721: F, t39723: F, t39728: F, t39732: F, t39737: F, t39741: F, t39744: F, t39747: F) -> (F, F, F, F, F, F, F) {
    let t39751 = t9017 * t358 * t363;
    let t39753 = t446 * t39749 * t39751;
    let t39755 = t1647 * t2075;
    let t39757 = t446 * t1969 * t39755;
    let t39759 = t1643 * t2075;
    let t39761 = t446 * t9049 * t39759;
    let t39763 = -12.0 * t39708 + 8.0 * t39711 - 4.0 * t39715 - 4.0 / 3.0 * t39717 + 8.0 * t39721 + 16.0 / 9.0 * t39723 + 40.0 / 27.0 * t39728 + 2.0 * t39732 + 4.0 / 3.0 * t39737 + 4.0 / 3.0 * t39741 + 8.0 / 3.0 * t39744 - 8.0 / 9.0 * t39747 + 8.0 * t39753 - 4.0 * t39757 + 4.0 / 3.0 * t39761;
    (t39751, t39753, t39755, t39757, t39759, t39761, t39763)
}
