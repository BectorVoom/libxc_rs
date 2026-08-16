//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2026/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2026<F: Float>(t94395: F, t94398: F, t4057: F, t676: F, t25880: F, t25904: F, t25945: F, t9285: F, t25944: F, t1364: F, t26075: F, t786: F) -> (F, F, F, F, F, F) {
    let t94399 = t94395 * t94398;
    let t94403 = t676 * t4057;
    let t94404 = t25880 * t94403;
    let t94405 = t25904 * t94404;
    let t94407 = t25945 * t9285;
    let t94409 = F::cast_from(0.68540937416128198417e-2_f64) * t25944 * t94407;
    let t94411 = t786 * t26075 * t1364;
    (t94399, t94404, t94405, t94407, t94409, t94411)
}
