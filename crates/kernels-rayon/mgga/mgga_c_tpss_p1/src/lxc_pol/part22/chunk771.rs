//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 771/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk771(t1072: f64, t1535: f64, t1080: f64, t1543: f64, t2836: f64, t2893: f64, t2981: f64, t2988: f64, t4044: f64, t4049: f64, t4054: f64, t4058: f64, t4072: f64, t4080: f64, t4088: f64, t4090: f64, t4093: f64, t4096: f64, t4099: f64, t4102: f64) -> (f64, f64, f64) {
    let t4158 = t1535 * t1072;
    let t4163 = t1543 * t1080;
    let t4180 = -0.1294625e1_f64 * t4072 + 0.258925e1_f64 * t4080 + t2981 - 0.10064166666666666667e0_f64 * t2836 - 0.10064166666666666667e0_f64 * t4044 - 0.20128333333333333333e0_f64 * t4049 + 0.60385e0_f64 * t4054 + 0.301925e0_f64 * t4058 + 0.82524375e-1_f64 * t4088 + 0.16504875e0_f64 * t4090 + t2988 - 0.5519e-1_f64 * t2893 - 0.5519e-1_f64 * t4093 - 0.27595e-1_f64 * t4096 + 0.16557e0_f64 * t4099 + 0.82785e-1_f64 * t4102;
    (t4158, t4163, t4180)
}
