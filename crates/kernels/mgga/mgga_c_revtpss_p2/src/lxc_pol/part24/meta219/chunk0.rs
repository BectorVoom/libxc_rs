//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 967/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk967<F: Float>(t271: F, t2857: F, t11144: F, t11150: F, t3252: F, t283: F, t66: F, t3298: F, t994: F, t4891: F, t3316: F, t11132: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11821 = F::new(1.0) / t271 / t2857;
    let t11822 = t11821 * t11144;
    let t11827 = t3252 * t11150;
    let t11852 = F::new(1.0) / t283 / t2857;
    let t11853 = t66 * t11852;
    let t11858 = t994 * t3298;
    let t11859 = t11858 * t4891;
    let t11874 = t994 * t3316;
    let t11875 = t11874 * t4891;
    let t11890 = F::cast_from(0.25925925925925925926e-1_f64) * t11132;
    (t11821, t11822, t11827, t11852, t11853, t11858, t11859, t11874, t11875, t11890)
}
