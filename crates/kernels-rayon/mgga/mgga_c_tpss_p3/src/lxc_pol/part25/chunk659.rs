//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 659/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk659(t2895: f64, t4047: f64, t141: f64, t1038: f64, t4052: f64, t4056: f64, t2836: f64, t2880: f64, t2892: f64, t2893: f64, t4044: f64, t4049: f64, t4054: f64, t4058: f64, t4072: f64, t4080: f64, t4088: f64, t4090: f64, t4093: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4095 = t2895 * t4047;
    let t4096 = t141 * t4095;
    let t4098 = t1038 * t4052;
    let t4099 = t141 * t4098;
    let t4101 = t1038 * t4056;
    let t4102 = t141 * t4101;
    let t4104 = -0.9494625e0_f64 * t4072 + 0.1898925e1_f64 * t4080 + t2880 - 0.99655555555555555557e-1_f64 * t2836 - 0.99655555555555555557e-1_f64 * t4044 - 0.19931111111111111111e0_f64 * t4049 + 0.59793333333333333334e0_f64 * t4054 + 0.29896666666666666667e0_f64 * t4058 + 0.15358125e0_f64 * t4088 + 0.3071625e0_f64 * t4090 + t2892 - 0.54771111111111111111e-1_f64 * t2893 - 0.54771111111111111111e-1_f64 * t4093 - 0.27385555555555555556e-1_f64 * t4096 + 0.16431333333333333333e0_f64 * t4099 + 0.82156666666666666667e-1_f64 * t4102;
    (t4095, t4096, t4098, t4099, t4101, t4102, t4104)
}
