//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1384/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1384<F: Float>(t1091: F, t24980: F, t24981: F, t28821: F, t113208: F, t3746: F, t7062: F, t113070: F, t4162: F, t28735: F, t31577: F, t684: F, t126799: F, t6317: F, t99391: F, t127796: F, t127800: F, t127803: F, t127806: F, t127808: F, t127812: F, t99825: F) -> (F, F, F, F, F, F) {
    let t127816 = t24980 * t24981 * t28821 * t1091;
    let t127820 = t113208 * t24981 * t7062 * t3746;
    let t127824 = t24980 * t113070 * t7062 * t4162;
    let t127828 = t28735 * t24981 * t31577 * t684;
    let t127831 = t6317 * t99391 * t126799;
    let t127833 = -6.0 * t127796 + 4.0 * t127800 + t99825 + 2.0 / 3.0 * t127803 + t127806 + 4.0 / 3.0 * t127808 - t127812 / 6.0 - t127816 / 6.0 + t127820 / 3.0 + 3.0 * t127824 + t127828 / 8.0 - 2.0 / 9.0 * t127831;
    (t127816, t127820, t127824, t127828, t127831, t127833)
}
