//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1089/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1089(t26725: f64, t26776: f64, t26822: f64, t26864: f64, t393: f64, t1141: f64, t7738: f64, t1203: f64, t2183: f64, t3329: f64, t3331: f64, t3481: f64, t7740: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26866 = t26725 + t26776 + t26822 + t26864;
    let t26867 = t26866 * t393;
    let t26868 = t7738 * t1141;
    let t26870 = 2.0_f64 * t26868 * t1203;
    let t26871 = t2183 * t3329;
    let t26873 = 2.0_f64 * t26871 * t3331;
    let t26874 = t7740 * t3481;
    (t26866, t26867, t26868, t26870, t26871, t26873, t26874)
}
