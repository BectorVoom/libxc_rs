//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1062/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1062<F: Float>(t12288: F, t7063: F, t1125: F, t3449: F, t2469: F, t11183: F, t11186: F, t12012: F, t12013: F, t12014: F, t12015: F, t12016: F, t12017: F, t12018: F, t12019: F, t12020: F, t12021: F, t12022: F) -> (F, F, F, F) {
    let t12290 = F::cast_from(6.0_f64) * t7063 * t12288;
    let t12291 = t1125 * t3449;
    let t12293 = F::cast_from(2.0_f64) * t2469 * t12291;
    let t12580 = F::cast_from(0.5431140175846100239e-5_f64) * t11183 + F::cast_from(0.5431140175846100239e-5_f64) * t11186 - t12012 - t12013 - t12014 + t12015 + t12016 - t12017 + t12018 - t12019 - t12020 + t12021 + t12022;
    (t12290, t12291, t12293, t12580)
}
