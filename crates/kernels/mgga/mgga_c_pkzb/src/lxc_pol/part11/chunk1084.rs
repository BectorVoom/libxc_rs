//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1084/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1084<F: Float>(t24662: F, t20366: F, t20368: F, t20370: F, t20374: F, t124: F, t28912: F, t16935: F, t10534: F, t114: F, t557: F, t24671: F, t20378: F, t16822: F, t16825: F, t16946: F, t16950: F, t20365: F, t20373: F, t20377: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t29139 = 0.54934341918019635162e-3 * t24662;
    let t29140 = 0.65061487801810439052e-1 * t20366;
    let t29141 = 0.97592231702715658578e-1 * t20368;
    let t29142 = 0.10526802520742363173e2 * t20370;
    let t29143 = 0.15584273195113317383e3 * t20374;
    let t29145 = 0.19751673498613801407e-1 * t28912 * t124;
    let t29146 = 0.56968947174242584612e-3 * t16935;
    let t29148 = t10534 * t114 * t557;
    let t29149 = 0.5848223622634646207e0 * t29148;
    let t29150 = 36.0 * t24671;
    let t29151 = 180.0 * t20378;
    let t29152 = -t16822 - t20365 - t29139 - t29140 - t29141 + t29142 + t20373 - t29143 + t29145 + t16825 - t29146 - t20377 + t16946 + t16950 - t29149 - t29150 + t29151;
    (t29139, t29140, t29141, t29142, t29143, t29145, t29146, t29149, t29150, t29151, t29152)
}
