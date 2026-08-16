//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1239/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1239(t19895: f64, t19897: f64, t19899: f64, t19902: f64, t19906: f64, t19909: f64, t19912: f64, t19914: f64, t19916: f64, t19918: f64, t19920: f64, t19922: f64, t19924: f64, t19927: f64, t19929: f64, t19932: f64, t19935: f64, t19938: f64, t19941: f64) -> f64 {
    let t20769 = -0.44965277777777777777e-2_f64 * t19895 - 0.14388888888888888889e0_f64 * t19897 - 0.1875e0_f64 * t19899 - 0.4046875e-1_f64 * t19902 - 0.101171875e-1_f64 * t19906 - 0.44965277777777777777e-2_f64 * t19909 + 0.23981481481481481481e-1_f64 * t19912 + 0.101171875e-1_f64 * t19914 + 0.20234375e-1_f64 * t19916 + 0.125e0_f64 * t19918 - 0.125e0_f64 * t19920 - 0.20234375e-1_f64 * t19922 - 0.9375e-1_f64 * t19924 - 0.125e0_f64 * t19927 - 0.10791666666666666667e0_f64 * t19929 - 0.5625e0_f64 * t19932 + 0.20833333333333333333e-1_f64 * t19935 + 0.27777777777777777777e-1_f64 * t19938 - 0.26979166666666666667e-1_f64 * t19941;
    t20769
}
