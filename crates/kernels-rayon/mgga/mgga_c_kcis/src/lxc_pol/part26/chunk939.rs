//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 939/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk939(t4142: f64, t6909: f64, t11882: f64, t12231: f64, t15983: f64, t15987: f64, t21811: f64, t21816: f64, t21819: f64, t21822: f64, t21825: f64, t21828: f64, t3961: f64) -> (f64, f64) {
    let t21834 = t4142 * t6909;
    let t21837 = -0.66327777777777777776e-2_f64 * t21811 + 0.55273148148148148147e-2_f64 * t21816 - 0.55273148148148148147e-3_f64 * t21819 + 0.49745833333333333332e-2_f64 * t21822 + 0.13265555555555555555e-1_f64 * t21825 - 0.2671335375e-1_f64 * t3961 * t21828 - 0.178244852896875e-2_f64 * t12231 * t21828 - 0.36848765432098765431e-3_f64 * t11882 - 0.58958024691358024689e-2_f64 * t21834 + 0.29479012345679012345e-2_f64 * t15983 - t15987;
    (t21834, t21837)
}
