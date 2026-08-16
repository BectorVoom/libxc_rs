//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1296/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1296<F: Float>(t213: F, t81968: F, t1894: F, t236: F, t9458: F, t81907: F, t81909: F, t81912: F, t81918: F, t81921: F, t81924: F, t81926: F, t81928: F, t81930: F, t81934: F, t81936: F, t81940: F, t81943: F, t81946: F, t81949: F, t81955: F, t81957: F, t81960: F, t81964: F) -> F {
    let t81969 = t81968 * t213;
    let t81972 = t81969 * t1894 * t236 * t9458;
    let t81974 = F::cast_from(0.12111826828242117256e-2_f64) * t81907 + F::cast_from(0.42391393898847410397e-2_f64) * t81909 - F::cast_from(0.33913115119077928317e-1_f64) * t81912 - F::cast_from(0.20186378047070195427e-3_f64) * t81918 - t81921 + F::cast_from(0.10093189023535097714e-3_f64) * t81924 - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t81926 + F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t81928 - t81930 / F::cast_from(48.0_f64) - F::cast_from(0.2034786907144675699e0_f64) * t81934 + F::cast_from(0.25434836339308446238e-1_f64) * t81936 - F::cast_from(0.12111826828242117256e-2_f64) * t81940 - F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t81943 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t81946 + F::cast_from(0.25434836339308446237e-1_f64) * t81949 - t81955 - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t81957 - t81960 / F::cast_from(4.0_f64) - F::cast_from(0.17804385437515912366e0_f64) * t81964 - F::cast_from(0.67826230238155856634e-1_f64) * t81972;
    t81974
}
