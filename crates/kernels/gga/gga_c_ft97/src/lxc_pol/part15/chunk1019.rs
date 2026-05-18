//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1019/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1019<F: Float>(t74690: F, t979: F, t4505: F, t38456: F, t91: F, t4533: F, t1766: F, t38549: F, t85469: F, t8314: F, t464: F, t85501: F) -> (F, F, F, F, F, F) {
    let t86010 = t74690 * t979;
    let t86014 = t4505 * t4505;
    let t86016 = t91 * t38456 * t86014;
    let t86018 = t4533 * t4533;
    let t86020 = t91 * t1766 * t86018;
    let t86023 = t38549 * t85469;
    let t86027 = t8314 * t85469;
    let t86031 = t464 * t85501;
    (t86010, t86016, t86020, t86023, t86027, t86031)
}
