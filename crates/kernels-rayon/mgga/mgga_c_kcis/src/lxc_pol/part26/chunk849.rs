//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 849/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk849(t17259: f64, t1928: f64, t4161: f64, t2820: f64, t5659: f64, t86: f64, t5664: f64, t11913: f64, t5656: f64, t5638: f64, t1924: f64, t3960: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17260 = 0.33163888888888888888e-2_f64 * t17259;
    let t17261 = t4161 * t1928;
    let t17266 = t86 * t2820 * t5659;
    let t17267 = t17266 * t5664;
    let t17268 = 0.3684876543209876543e-2_f64 * t17267;
    let t17274 = t11913 * t5656;
    let t17276 = t11913 * t5638;
    let t17277 = 0.14739506172839506172e-2_f64 * t17276;
    let t17287 = t1924 * t3960;
    (t17260, t17261, t17267, t17268, t17274, t17276, t17277, t17287)
}
