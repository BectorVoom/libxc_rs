//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 679/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk679<F: Float>(t3104: F, t8693: F, t1707: F, t3103: F, t1504: F, t1720: F, t126: F, t417: F, t581: F, t3105: F, t5395: F, t8624: F, t5727: F, t5743: F, t5692: F) -> (F, F, F, F, F, F, F) {
    let t8694 = t3104 * t8693;
    let t8696 = t1707 * t3103;
    let t8697 = t1720 * t1504;
    let t8698 = t8696 * t8697;
    let t8700 = t126 * t417;
    let t8701 = t581 * t8700;
    let t8702 = t8701 * t3105;
    let t8704 = t5395 * t8624;
    let t8705 = t8704 * t5727;
    let t8707 = t8704 * t5743;
    let t8709 = 1.0 / t5692;
    (t8694, t8698, t8700, t8702, t8705, t8707, t8709)
}
