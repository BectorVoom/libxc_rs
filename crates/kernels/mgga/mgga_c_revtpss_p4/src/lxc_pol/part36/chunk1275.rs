//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1275/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1275<F: Float>(t106430: F, t25431: F, t27341: F, t99463: F, t99466: F, t2411: F, t29704: F, t30088: F, t689: F, t25904: F, t25899: F, t30105: F) -> (F, F, F, F, F, F, F) {
    let t106433 = t25431 * t106430;
    let t106446 = t99463 * t27341;
    let t106448 = t99466 * t27341;
    let t106516 = t29704 * t2411;
    let t108132 = t30088 * t689;
    let t108133 = t25904 * t108132;
    let t108135 = t25899 * t108132;
    let t108138 = t30105 * t689;
    (t106433, t106446, t106448, t106516, t108133, t108135, t108138)
}
