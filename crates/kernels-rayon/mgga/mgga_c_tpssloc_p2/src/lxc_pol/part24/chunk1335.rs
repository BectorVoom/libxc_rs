//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1335/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1335(t213: f64, t81968: f64, t1894: f64, t236: f64, t9458: f64, t81907: f64, t81909: f64, t81912: f64, t81918: f64, t81921: f64, t81924: f64, t81926: f64, t81928: f64, t81930: f64, t81934: f64, t81936: f64, t81940: f64, t81943: f64, t81946: f64, t81949: f64, t81955: f64, t81957: f64, t81960: f64, t81964: f64) -> f64 {
    let t81969 = t81968 * t213;
    let t81972 = t81969 * t1894 * t236 * t9458;
    let t81974 = 0.12111826828242117256e-2_f64 * t81907 + 0.42391393898847410397e-2_f64 * t81909 - 0.33913115119077928317e-1_f64 * t81912 - 0.20186378047070195427e-3_f64 * t81918 - t81921 + 0.10093189023535097714e-3_f64 * t81924 - 7.0_f64 / 768.0_f64 * t81926 + 119.0_f64 / 2304.0_f64 * t81928 - t81930 / 48.0_f64 - 0.2034786907144675699e0_f64 * t81934 + 0.25434836339308446238e-1_f64 * t81936 - 0.12111826828242117256e-2_f64 * t81940 - 35.0_f64 / 72.0_f64 * t81943 + 3.0_f64 / 16.0_f64 * t81946 + 0.25434836339308446237e-1_f64 * t81949 - t81955 - 7.0_f64 / 16.0_f64 * t81957 - t81960 / 4.0_f64 - 0.17804385437515912366e0_f64 * t81964 - 0.67826230238155856634e-1_f64 * t81972;
    t81974
}
