//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1276/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1276(t12743: f64, t1750: f64, t14220: f64, t6144: f64, t1165: f64, t1180: f64, t14044: f64, t14048: f64, t14054: f64, t14059: f64, t18159: f64, t18164: f64, t18166: f64, t18176: f64, t18189: f64, t1884: f64, t3403: f64, t4437: f64, t5922: f64, t955: f64) -> f64 {
    let t23593 = t12743 * t1750;
    let t23606 = t14220 * t6144;
    let t23614 = 0.45351183609335988442e-1_f64 * t23593 + 0.40015750243531754508e-2_f64 * t18159 + 0.21437009059034868486e-3_f64 * t14044 - 0.42874018118069736972e-2_f64 * t3403 * t1165 * t1884 * t955 - 0.13719685797782315831e-1_f64 * t18164 - 0.64025200389650807212e-1_f64 * t18166 - 0.68598428988911579156e-2_f64 * t14048 - 0.12004725073059526352e-1_f64 * t14054 + 0.68598428988911579156e-2_f64 * t14059 - 0.16006300097412701803e-1_f64 * t23606 - 0.85748036236139473944e-3_f64 * t18176 - 0.17149607247227894789e-1_f64 * t18189 + 0.42874018118069736972e-3_f64 * t1180 * t1165 * t5922 * t4437;
    t23614
}
