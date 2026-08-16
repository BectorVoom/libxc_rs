//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1193/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1193(t35348: f64, t35359: f64, t31108: f64, t31111: f64, t31118: f64, t31120: f64, t31124: f64, t31126: f64, t31128: f64, t31131: f64, t31140: f64, t31143: f64, t31160: f64, t31162: f64, t35342: f64, t35350: f64, t35357: f64, t35366: f64) -> f64 {
    let t37498 = 0.14291339372689912324e-2_f64 * t35348;
    let t37504 = 0.39221875e0_f64 * t35359;
    let t37510 = 0.61125e-1_f64 * t31108 - 7.0_f64 / 24.0_f64 * t31111 - 0.42874018118069736972e-2_f64 * t35342 + 0.31448092289604152068e-2_f64 * t31118 - 0.37737710747524982483e-2_f64 * t31120 - 0.6289618457920830414e-2_f64 * t31124 - t37498 - 0.85748036236139473944e-3_f64 * t35350 + 0.264875e0_f64 * t31126 - 0.11433071498151929859e-2_f64 * t31128 - t31131 / 32.0_f64 + 0.1528125e-1_f64 * t35357 + t37504 + 0.305625e-1_f64 * t31140 - 7.0_f64 / 36.0_f64 * t31143 - 0.68598428988911579156e-2_f64 * t31160 - t35366 / 2.0_f64 + 0.25724410870841842184e-2_f64 * t31162;
    t37510
}
