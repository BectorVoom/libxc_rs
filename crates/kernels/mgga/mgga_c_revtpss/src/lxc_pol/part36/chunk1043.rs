//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1043/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1043<F: Float>(t1549: F, t25277: F, t25234: F, t4349: F, t25227: F, t4353: F, t2661: F, t1565: F, t25222: F, t241: F, t25260: F, t820: F, t72: F, t7778: F, t686: F, t7064: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27246 = t25277 * t1549;
    let t27251 = t25234 * t4349;
    let t27253 = t25227 * t4353;
    let t27254 = t2661 * t27253;
    let t27256 = t25222 * t1565;
    let t27261 = t820 * t25260 * t241;
    let t27278 = t7778 * t72;
    let t27279 = t27278 * t686;
    let t27280 = t7064 * t27279;
    (t27246, t27251, t27253, t27254, t27256, t27261, t27278, t27279, t27280)
}
