//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2015/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2015(t99066: f64, t99069: f64, t99073: f64, t99077: f64, t93004: f64, t93010: f64, t93016: f64, t95678: f64, t95680: f64, t95684: f64, t99063: f64, t99071: f64, t99075: f64) -> f64 {
    let t103315 = 0.16006300097412701803e0_f64 * t99066;
    let t103316 = 0.11433071498151929859e-3_f64 * t99069;
    let t103318 = 0.2032800112371413129e-2_f64 * t99073;
    let t103320 = 0.10164000561857065645e-3_f64 * t99077;
    let t103321 = 0.11433071498151929859e-3_f64 * t93004 + t95678 - 0.57165357490759649295e-3_f64 * t93010 - t99063 / 2.0_f64 - t95680 - 0.36143185997963725434e-4_f64 * t93016 - t95684 - t103315 - t103316 + 0.34299214494455789578e-1_f64 * t99071 + t103318 - 0.85748036236139473944e-3_f64 * t99075 - t103320;
    t103321
}
