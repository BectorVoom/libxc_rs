//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1046/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1046<F: Float>(t1477: F, t5422: F, t193: F, t1253: F, t7022: F, t31551: F, t798: F, t317: F, t5299: F, t6222: F, t28835: F, t6970: F, t1212: F, t296: F, t31358: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t31664 = t1477 * t5422;
    let t31665 = t193 * t31664;
    let t31668 = t7022 * t1253;
    let t31669 = t193 * t31668;
    let t31672 = t798 * t31551;
    let t31673 = t31672 * t317;
    let t31674 = t193 * t31673;
    let t31677 = t317 * t5299;
    let t31678 = t6222 * t31677;
    let t31679 = t193 * t31678;
    let t31682 = t28835 * t6970;
    let t31683 = t193 * t31682;
    let t31686 = t1253 * t1212;
    let t31687 = t6222 * t31686;
    let t31688 = t193 * t31687;
    let t31691 = t296 * t31358;
    (t31664, t31665, t31668, t31669, t31672, t31673, t31674, t31677, t31678, t31679, t31682, t31683, t31686, t31687, t31688, t31691)
}
