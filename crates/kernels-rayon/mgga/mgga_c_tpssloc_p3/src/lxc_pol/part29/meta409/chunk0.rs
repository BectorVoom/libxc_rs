//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1662/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1662(t11975: f64, t11977: f64, t11981: f64, t2528: f64, t5154: f64, t172: f64, t5151: f64, t763: f64, t2535: f64, t5166: f64, t592: f64, t12461: f64, t1845: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15887 = 4.0_f64 * t11975;
    let t15888 = 4.0_f64 * t11977;
    let t15889 = 32.0_f64 * t11981;
    let t15890 = t5154 * t2528;
    let t15891 = 0.17315859105681463759e2_f64 * t15890;
    let t15892 = t5151 * t172;
    let t15894 = 0.11696447245269292414e1_f64 * t15892 * t763;
    let t15895 = t5154 * t2535;
    let t15896 = 0.5848223622634646207e0_f64 * t15895;
    let t15898 = 8.0_f64 * t592 * t5166;
    let t15899 = t1845 * t12461;
    (t15887, t15888, t15889, t15891, t15894, t15896, t15898, t15899)
}
