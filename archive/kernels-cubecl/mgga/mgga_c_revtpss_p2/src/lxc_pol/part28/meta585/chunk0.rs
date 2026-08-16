//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2050/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2050<F: Float>(t1444: F, t543: F, t268: F, t4102: F, t94395: F, t4057: F, t676: F, t25880: F, t25904: F, t25945: F, t9285: F, t25944: F) -> (F, F, F, F, F, F) {
    let t94396 = t543 * t1444;
    let t94398 = t268 * t4102 * t94396;
    let t94399 = t94395 * t94398;
    let t94403 = t676 * t4057;
    let t94404 = t25880 * t94403;
    let t94405 = t25904 * t94404;
    let t94407 = t25945 * t9285;
    let t94409 = F::cast_from(0.68540937416128198417e-2_f64) * t25944 * t94407;
    (t94398, t94399, t94404, t94405, t94407, t94409)
}
