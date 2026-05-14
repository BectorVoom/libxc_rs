//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 744/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk744<F: Float>(t4458: F, t7025: F, t1549: F, t25277: F, t4345: F, t7045: F, t25234: F, t4349: F, t25227: F, t4353: F, t2661: F, t1565: F, t25222: F, t241: F, t25260: F, t820: F) -> (F, F, F, F, F, F, F) {
    let t27244 = t7025 * t4458;
    let t27246 = t25277 * t1549;
    let t27249 = t7045 * t4345;
    let t27251 = t25234 * t4349;
    let t27253 = t25227 * t4353;
    let t27254 = t2661 * t27253;
    let t27256 = t25222 * t1565;
    let t27261 = t820 * t25260 * t241;
    (t27244, t27246, t27249, t27251, t27254, t27256, t27261)
}
