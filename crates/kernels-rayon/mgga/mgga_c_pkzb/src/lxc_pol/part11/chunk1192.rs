//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1192/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1192(t24662: f64, t20366: f64, t20368: f64, t20370: f64, t20374: f64, t124: f64, t28912: f64, t16935: f64, t10534: f64, t114: f64, t557: f64, t24671: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29139 = 0.54934341918019635162e-3_f64 * t24662;
    let t29140 = 0.65061487801810439052e-1_f64 * t20366;
    let t29141 = 0.97592231702715658578e-1_f64 * t20368;
    let t29142 = 0.10526802520742363173e2_f64 * t20370;
    let t29143 = 0.15584273195113317383e3_f64 * t20374;
    let t29145 = 0.19751673498613801407e-1_f64 * t28912 * t124;
    let t29146 = 0.56968947174242584612e-3_f64 * t16935;
    let t29148 = t10534 * t114 * t557;
    let t29149 = 0.5848223622634646207e0_f64 * t29148;
    let t29150 = 36.0_f64 * t24671;
    (t29139, t29140, t29141, t29142, t29143, t29145, t29146, t29149, t29150)
}
