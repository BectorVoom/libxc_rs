//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 958/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk958<F: Float>(t283: F, t2857: F, t3298: F, t994: F, t4891: F, t3154: F, t999: F, t1086: F, t3046: F, t3090: F, t3316: F, t1016: F, t697: F, t1011: F, t11132: F, t126: F, t373: F) -> (F, F, F, F, F, F, F, F) {
    let t11852 = 1.0 / t283 / t2857;
    let t11858 = t994 * t3298;
    let t11859 = t11858 * t4891;
    let t11860 = t3154 * t999;
    let t11865 = t3046 * t1086;
    let t11866 = t11865 * t3090;
    let t11874 = t994 * t3316;
    let t11875 = t11874 * t4891;
    let t11880 = t697 * t1016;
    let t11881 = t1011 * t11880;
    let t11890 = 0.25925925925925925926e-1 * t11132;
    let t11921 = t126 * t373;
    (t11852, t11859, t11860, t11866, t11875, t11881, t11890, t11921)
}
