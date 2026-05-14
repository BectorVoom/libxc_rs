//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1131/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1131<F: Float>(t1450: F, t2014: F, t532: F, t94588: F, t94637: F, t94692: F, t94744: F, t94794: F, t94846: F, t94893: F, t94934: F, t25194: F, t7235: F, t2034: F, t46304: F, t1936: F, t46126: F) -> (F, F, F, F) {
    let t94940 = t2014 * t532 * (t94588 + t94637 + t94692 + t94744 + t94794 + t94846 + t94893 + t94934) * t1450;
    let t94942 = 6.0 * t7235 * t25194;
    let t94944 = t2014 * t2034 * t46304;
    let t94956 = 2.0 * t46126 * t1936;
    (t94940, t94942, t94944, t94956)
}
