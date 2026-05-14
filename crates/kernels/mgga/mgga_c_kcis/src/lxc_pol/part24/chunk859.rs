//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 859/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk859<F: Float>(t19926: F, t3337: F, t5096: F, t5172: F, t19609: F, t5047: F, t14832: F, t19593: F, t5077: F, t14838: F, t19588: F, t5076: F, t19614: F, t3438: F, t5175: F, t19895: F, t19897: F, t19899: F, t19902: F, t19906: F, t19909: F, t19912: F, t19914: F, t19916: F, t19918: F, t19920: F, t19922: F, t19924: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t19927 = t3337 * t19926;
    let t19929 = t5172 * t5096;
    let t19931 = t5047 * t19609;
    let t19932 = t14832 * t19931;
    let t19934 = t5077 * t19593;
    let t19935 = t3337 * t19934;
    let t19937 = t14838 * t19588;
    let t19938 = t5076 * t19937;
    let t19940 = t3438 * t19614;
    let t19941 = t5175 * t19940;
    let t19943 = -t19895 / 576.0 - t19897 / 18.0 - t19899 / 8.0 - t19902 / 64.0 - t19906 / 256.0 - t19909 / 576.0 + t19912 / 108.0 + t19914 / 256.0 + t19916 / 128.0 + t19918 / 12.0 - t19920 / 12.0 - t19922 / 128.0 - t19924 / 16.0 - t19927 / 12.0 - t19929 / 24.0 - 3.0 / 8.0 * t19932 + t19935 / 72.0 + t19938 / 54.0 - t19941 / 96.0;
    (t19927, t19929, t19931, t19932, t19934, t19935, t19937, t19938, t19940, t19941, t19943)
}
