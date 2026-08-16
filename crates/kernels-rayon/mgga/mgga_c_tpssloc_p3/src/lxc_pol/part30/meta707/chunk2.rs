//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2334/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2334(t3941: f64, t4072: f64, t7467: f64, t28017: f64, t3938: f64, t12524: f64, t28899: f64, t100867: f64, t100871: f64, t100873: f64, t100875: f64, t100879: f64, t100883: f64, t100885: f64, t100887: f64, t100890: f64, t20176: f64, t23877: f64, t23880: f64, t26523: f64, t5456: f64, t5493: f64, t577: f64, t83980: f64, t96351: f64) -> f64 {
    let t100893 = 54.0_f64 * t3941 * t7467 * t4072;
    let t100897 = 0.135e2_f64 * t3938 * t28017;
    let t100899 = 27.0_f64 * t12524 * t28899;
    let t100900 = 27.0_f64 * t26523 * t4072 + 27.0_f64 * t96351 * t5456 + 0.45e1_f64 * t100867 * t577 + t100871 + t100873 + t100875 + 54.0_f64 * t23880 * t20176 + t100879 + 27.0_f64 * t83980 * t5456 + t100883 + t100885 + t100887 + t100890 + t100893 + 0.135e2_f64 * t23877 * t5493 + t100897 + t100899;
    t100900
}
