//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 800/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk800(t5371: f64, t7467: f64, t5456: f64, t576: f64, t1873: f64, t1458: f64, t3941: f64, t5493: f64, t1401: f64, t28017: f64, t2031: f64, t27956: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28892 = 27.0_f64 * t5371 * t7467;
    let t28893 = t576 * t5456;
    let t28895 = 27.0_f64 * t28893 * t1873;
    let t28896 = t7467 * t1458;
    let t28898 = 54.0_f64 * t3941 * t28896;
    let t28899 = t1873 * t5493;
    let t28901 = 27.0_f64 * t3941 * t28899;
    let t28903 = 0.135e2_f64 * t1401 * t28017;
    let t28935 = t2031 * t27956;
    (t28892, t28893, t28895, t28896, t28898, t28899, t28901, t28903, t28935)
}
