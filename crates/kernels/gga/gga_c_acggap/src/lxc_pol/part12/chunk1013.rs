//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1013/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1013<F: Float>(t34081: F, t34091: F, t34095: F, t34099: F, t34101: F, t34107: F, t30269: F, t30273: F, t30280: F, t30297: F, t34072: F, t34074: F, t34078: F, t34085: F, t34089: F, t34105: F, t34111: F, t34115: F) -> (F,) {
    let t36934 = 0.31448092289604152068e-2 * t34081;
    let t36937 = 0.34299214494455789578e-2 * t34091;
    let t36938 = 0.12579236915841660828e-2 * t34095;
    let t36939 = 0.42874018118069736972e-2 * t34099;
    let t36940 = 0.21437009059034868486e-2 * t34101;
    let t36942 = 0.18868855373762491241e-1 * t34107;
    let t36945 = 0.18868855373762491241e-1 * t30269 - 0.13719685797782315831e-1 * t34072 + 0.68598428988911579156e-2 * t34074 + 0.42874018118069736972e-3 * t30273 + 0.57165357490759649296e-3 * t30280 - 0.68598428988911579156e-2 * t34078 - 0.42874018118069736972e-2 * t30297 - t36934 - 0.31448092289604152068e-2 * t34085 - 0.21437009059034868486e-2 * t34089 + t36937 - t36938 - t36939 + t36940 - 0.18868855373762491241e-1 * t34105 + t36942 + 0.21437009059034868486e-2 * t34111 + 0.85748036236139473944e-3 * t34115;
    (t36945,)
}
