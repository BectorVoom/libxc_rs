//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1193/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1193(t19895: f64, t19897: f64, t19899: f64, t19902: f64, t19906: f64, t19909: f64, t19912: f64, t19914: f64, t19916: f64, t19918: f64, t19920: f64, t19922: f64, t19924: f64, t19927: f64, t19929: f64, t19932: f64, t19935: f64, t19938: f64, t19941: f64) -> f64 {
    let t19943 = -t19895 / 576.0_f64 - t19897 / 18.0_f64 - t19899 / 8.0_f64 - t19902 / 64.0_f64 - t19906 / 256.0_f64 - t19909 / 576.0_f64 + t19912 / 108.0_f64 + t19914 / 256.0_f64 + t19916 / 128.0_f64 + t19918 / 12.0_f64 - t19920 / 12.0_f64 - t19922 / 128.0_f64 - t19924 / 16.0_f64 - t19927 / 12.0_f64 - t19929 / 24.0_f64 - 3.0_f64 / 8.0_f64 * t19932 + t19935 / 72.0_f64 + t19938 / 54.0_f64 - t19941 / 96.0_f64;
    t19943
}
