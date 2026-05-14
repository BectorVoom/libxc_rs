//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 465/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk465<F: Float>(t2832: F, t312: F, t2649: F, t2745: F, t2750: F, t2802: F, t2845: F, t2892: F, t301: F, t317: F, t830: F, t880: F, t332: F, t5: F, t885: F, t170: F, t2248: F, t328: F) -> (F, F, F, F, F) {
    let t2894 = t2832 * t312;
    let t2899 = -t2649 * t317 - t2745 * t317 - t2892 * t301 - 2.0 * t830 * t880 - 4.0 * t2750 - 2.0 * t2802 + 4.0 * t2845 + 2.0 * t2894;
    let t2900 = t2899 * t332;
    let t2904 = t5 * t885;
    let t2912 = 5.0 / 18.0 * t170 * t2248 * t328;
    (t2894, t2899, t2900, t2904, t2912)
}
