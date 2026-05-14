//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1243/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1243<F: Float>(t123890: F, t1434: F, t193: F, t9942: F, t18524: F, t6135: F, t3281: F, t9770: F, t5120: F, t6061: F, t6109: F, t743: F, t123859: F, t123863: F, t123867: F, t123870: F, t123872: F, t123876: F, t123881: F, t123885: F, t123888: F) -> (F, F, F, F, F) {
    let t123893 = t1434 * t193 * t9942 * t123890;
    let t123894 = t6135 * t18524;
    let t123896 = t3281 * t9770 * t123894;
    let t123901 = t6109 * t193 * t743 * t6061 * t5120;
    let t123903 = t123859 / 2.0 + t123863 / 9.0 + 2.0 / 27.0 * t123867 + t123870 / 18.0 + 2.0 / 27.0 * t123872 - 4.0 / 9.0 * t123876 + t123881 / 12.0 - 4.0 / 9.0 * t123885 + 4.0 / 27.0 * t123888 - t123893 + 8.0 / 9.0 * t123896 + t123901 / 12.0;
    (t123893, t123894, t123896, t123901, t123903)
}
