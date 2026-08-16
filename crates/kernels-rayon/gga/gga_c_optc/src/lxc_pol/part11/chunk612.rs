//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 612/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk612(t5064: f64, t914: f64, t2549: f64, t4768: f64, t1000: f64, t4772: f64, t1435: f64, t2354: f64, t2569: f64, t277: f64, t3975: f64, t4054: f64, t4897: f64, t4952: f64, t4956: f64, t4960: f64, t5059: f64, t95: f64, t999: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5065 = t914 * t5064;
    let t5068 = t2549 * t4768;
    let t5069 = t914 * t5068;
    let t5075 = t1000 * t4772;
    let t5076 = t914 * t5075;
    let t5079 = -t4952 - t4956 - t4960 - 0.25844881434903430496e-2_f64 * t95 * t277 * t5059 * t2569 - t4897 + t999 * t5065 / 6.0_f64 + 2.0_f64 / 9.0_f64 * t999 * t5069 + t4054 * t1435 / 3.0_f64 - t2354 + t3975 / 9.0_f64 - t999 * t5076 / 3.0_f64;
    (t5065, t5068, t5069, t5075, t5076, t5079)
}
