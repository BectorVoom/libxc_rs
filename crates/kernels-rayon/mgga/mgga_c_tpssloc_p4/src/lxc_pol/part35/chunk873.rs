//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 873/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk873(t10471: f64, t11715: f64, t11712: f64, t11721: f64, t6739: f64, t3502: f64, t3508: f64, t1209: f64, t475: f64, t3639: f64, t500: f64, t1287: f64, t2223: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11880 = t10471 * t11715;
    let t11881 = t11712 * t11880;
    let t11883 = t6739 * t11721;
    let t11887 = t10471 * t3502;
    let t11888 = t11712 * t11887;
    let t11889 = t6739 * t3508;
    let t11913 = t10471 * t1209;
    let t11914 = t11712 * t11913;
    let t11915 = t6739 * t475;
    let t11947 = 1.0_f64 / t3639 / t500;
    let t11981 = t2223 * t1287;
    (t11881, t11883, t11888, t11889, t11914, t11915, t11947, t11981)
}
