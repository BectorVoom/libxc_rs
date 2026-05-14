//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 703/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk703<F: Float>(t12001: F, t3283: F, t1643: F, t3199: F, t8518: F, t1651: F, t3182: F, t1909: F, t3193: F, t11982: F, t3187: F, t103: F, t7800: F, t11437: F, t3103: F, t379: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12002 = t12001 * t3283;
    let t12004 = t3199 * t1643;
    let t12005 = t8518 * t12004;
    let t12008 = t3182 * t1651;
    let t12009 = t1909 * t12008;
    let t12012 = t3182 * t1643;
    let t12013 = t3193 * t12012;
    let t12016 = t3187 * t11982;
    let t12017 = t1909 * t12016;
    let t12020 = t103 * t7800;
    let t12021 = t12020 * t11437;
    let t12022 = t3193 * t12021;
    let t12025 = t103 * t3103;
    let t12026 = t12025 * t379;
    (t12002, t12004, t12005, t12008, t12009, t12012, t12013, t12016, t12017, t12021, t12022, t12026)
}
