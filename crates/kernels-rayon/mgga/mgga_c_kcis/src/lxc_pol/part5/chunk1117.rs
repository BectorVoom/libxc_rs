//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1117/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1117(t10218: f64, t13710: f64, t13714: f64, t13717: f64, t13781: f64, t18828: f64, t18830: f64, t18833: f64, t18835: f64, t18853: f64, t9691: f64, t10199: f64, t10202: f64, t1036: f64, t13747: f64, t13750: f64, t1670: f64, t18685: f64, t18803: f64, t18808: f64, t18817: f64, t18824: f64, t245: f64, t3078: f64, t3081: f64, t4625: f64, t4647: f64, t4654: f64, t6320: f64, t6338: f64, t934: f64) -> f64 {
    let t18854 = 0.14865e-1_f64 * t18828 - 0.1982e-1_f64 * t18830 - 0.991e-2_f64 * t18833 + 0.1982e-1_f64 * t18835 - t10218 - 0.18344444444444444444e-2_f64 * t9691 - 0.36688888888888888888e-2_f64 * t13710 + t13781 - 0.55033333333333333332e-2_f64 * t13714 + 0.55033333333333333332e-2_f64 * t13717 + t18853;
    let t18857 = 3.0_f64 / 16.0_f64 * t10199 * t18803 - t10202 * t6320 / 8.0_f64 - t3078 * t18808 / 4.0_f64 - t13747 * t4647 / 4.0_f64 + t13750 * t1670 / 2.0_f64 + t4654 * t4625 / 2.0_f64 - t3078 * t18817 / 8.0_f64 + t3081 * t6338 / 4.0_f64 + t1036 * t18685 / 4.0_f64 + t18824 * t934 / 4.0_f64 + t245 * t18854 / 2.0_f64;
    t18857
}
