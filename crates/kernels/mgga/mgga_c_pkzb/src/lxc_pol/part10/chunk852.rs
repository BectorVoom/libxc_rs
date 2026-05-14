//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 852/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk852<F: Float>(t1485: F, t178: F, t301: F, t299: F, t2070: F, t771: F, t2003: F, t53: F, t179: F, t1885: F, t2002: F, t208: F, t154: F, t1843: F, t2048: F, t276: F) -> (F, F, F, F, F, F, F) {
    let t5612 = t178 * t1485 * t301;
    let t5614 = 0.63517063878621832551e-4 * t299 * t5612;
    let t5620 = t771 * t2070;
    let t5627 = t53 * t2003;
    let t5629 = t179 * t5627 * t1885;
    let t5630 = t299 * t5629;
    let t5633 = 1.0 / t2002 / t208;
    let t5645 = t154 * t2048 * t1843;
    let t5646 = t276 * t5645;
    (t5612, t5614, t5620, t5627, t5630, t5633, t5646)
}
