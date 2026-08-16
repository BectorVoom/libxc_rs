//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 781/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk781<F: Float>(t1030: F, t8986: F, t3076: F, t1795: F, t3104: F, t1636: F, t189: F, t185: F, t1771: F, t1723: F, t8770: F, t654: F, t8768: F) -> (F, F, F, F, F, F, F) {
    let t8987 = t1030 * t8986;
    let t8988 = t8987 * t3076;
    let t8990 = t3104 * t1795;
    let t8992 = t189 * t1636;
    let t8993 = t185 * t8992;
    let t8994 = t8993 * t1771;
    let t8996 = t8770 * t1723;
    let t8998 = t654 * t8768;
    (t8987, t8988, t8990, t8992, t8994, t8996, t8998)
}
