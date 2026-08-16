//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 566/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk566(t10856: f64, t2365: f64, t2033: f64, t10847: f64, t6066: f64, t7630: f64, t8521: f64, t959: f64, t2660: f64, t8793: f64, t787: f64, t8792: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10857 = t2365 * t10856;
    let t10858 = t2033 * t10857;
    let t10859 = 0.14896037479937677779e-1_f64 * t10858;
    let t10860 = t6066 * t10847;
    let t10862 = 0.71500979903700853338e0_f64 * t7630 * t10860;
    let t10863 = t8521 * t959;
    let t10864 = 0.14896037479937677779e-1_f64 * t10863;
    let t10866 = 0.10725146985555128001e1_f64 * t8793 * t2660;
    let t10867 = t787 * t8792;
    (t10858, t10859, t10862, t10863, t10864, t10866, t10867)
}
