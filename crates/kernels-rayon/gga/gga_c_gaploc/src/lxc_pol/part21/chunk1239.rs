//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1239/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1239(t32948: f64, t7427: f64, t7573: f64, t22333: f64, t24344: f64, t10889: f64, t23176: f64, t2017: f64, t3488: f64, t825: f64, t22909: f64, t25462: f64, t787: f64, t9824: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32951 = 0.12423108009070322895e3_f64 * t7427 * t7573 * t32948;
    let t32952 = t24344 * t22333;
    let t32953 = 0.29792074959875355558e-1_f64 * t32952;
    let t32954 = t10889 * t23176;
    let t32955 = 0.59584149919750711116e-1_f64 * t32954;
    let t32957 = t825 * t2017 * t3488;
    let t32958 = 0.59644551483876721719e0_f64 * t32957;
    let t32959 = t10889 * t22909;
    let t32960 = 0.14896037479937677779e-1_f64 * t32959;
    let t32962 = t787 * t25462 * t9824;
    (t32951, t32953, t32955, t32958, t32960, t32962)
}
