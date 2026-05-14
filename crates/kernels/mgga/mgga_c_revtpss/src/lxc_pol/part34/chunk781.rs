//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 781/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk781<F: Float>(t11772: F, t3088: F, t3114: F, t271: F, t2857: F, t11144: F, t11150: F, t3252: F, t283: F, t66: F, t3298: F, t994: F, t4891: F, t3316: F, t11132: F, t126: F, t373: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11773 = t3088 * t11772;
    let t11774 = t3114 * t11773;
    let t11821 = 1.0 / t271 / t2857;
    let t11822 = t11821 * t11144;
    let t11827 = t3252 * t11150;
    let t11852 = 1.0 / t283 / t2857;
    let t11853 = t66 * t11852;
    let t11858 = t994 * t3298;
    let t11859 = t11858 * t4891;
    let t11874 = t994 * t3316;
    let t11875 = t11874 * t4891;
    let t11890 = 0.25925925925925925926e-1 * t11132;
    let t11921 = t126 * t373;
    (t11774, t11822, t11827, t11853, t11858, t11859, t11874, t11875, t11890, t11921)
}
