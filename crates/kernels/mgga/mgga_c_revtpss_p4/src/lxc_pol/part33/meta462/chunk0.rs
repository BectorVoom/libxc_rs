//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1676/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1676<F: Float>(t1284: F, t6564: F, t6688: F, t73: F, t5458: F, t1287: F, t21257: F, t1811: F, t3766: F, t460: F, t3781: F, t21040: F) -> (F, F, F, F, F, F) {
    let t21439 = t6564 * t1284;
    let t21442 = t6688 * t73;
    let t21443 = t21442 * t5458;
    let t21448 = t21257 * t1287;
    let t21451 = t3766 * t1811;
    let t21452 = t460 * t21451;
    let t21455 = t3781 * t1811;
    let t21456 = t460 * t21455;
    let t21459 = t21040 * t5458;
    (t21439, t21443, t21448, t21452, t21456, t21459)
}
