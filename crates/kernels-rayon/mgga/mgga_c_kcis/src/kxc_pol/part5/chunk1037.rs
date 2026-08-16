//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1037/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1037(t1396: f64, t4161: f64, t12240: f64, t4142: f64, t5770: f64, t1017: f64, t541: f64, t86: f64, t3728: f64, t5882: f64, t5678: f64, t1494: f64, t2001: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15878 = t4161 * t1396;
    let t15887 = t12240 * t1396;
    let t15896 = t4142 * t5770;
    let t15909 = t86 * t1017 * t541;
    let t15934 = t3728 * t5882;
    let t15941 = t3728 * t5678;
    let t15942 = 0.66327777777777777776e-2_f64 * t15941;
    let t15955 = t1494 * t2001;
    (t15878, t15887, t15896, t15909, t15934, t15941, t15942, t15955)
}
