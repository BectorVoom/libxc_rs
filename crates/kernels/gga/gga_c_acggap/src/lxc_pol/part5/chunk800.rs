//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 800/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk800<F: Float>(t743: F, t746: F, t11735: F, t11747: F, t11756: F, t11770: F, t11775: F, t11792: F, t11825: F, t11828: F, t11834: F, t11837: F, t11869: F, t11870: F, t12015: F, t256: F, t257: F, t2754: F, t2755: F, t2768: F, t2795: F, t62: F, t680: F, t690: F, t728: F, t739: F, t745: F, t747: F, t748: F, t75: F, t793: F, t799: F) -> (F,) {
    let t12083 = t743 * t743;
    let t12086 = t746 * t746;
    let t12091 = -24.0 * t2768 * t12015 * t257 + 0.51947577317044391277e2 * t799 * t11735 * t690 - t11747 + t11756 - 0.12304822629859687989e5 * t75 * t11869 * t11870 * t2795 - t11770 + t11775 - t11792 - t11825 - t11828 - 0.11579025239058625248e4 * t2768 * t748 * t739 + 0.21053605041484726346e2 * t799 * t793 * t680 + 0.12865583598954028054e3 * t745 * t2754 * t747 * t256 - 8.0 * t728 * t2755 * t256 + t11834 - t11837 + 0.19964560303604640732e6 * t62 / t12083 * t12015 / t12086;
    (t12091,)
}
