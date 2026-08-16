//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1217/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1217(t36236: f64, t36238: f64, t36240: f64, t36273: f64, t36283: f64, t36286: f64, t36243: f64, t36246: f64, t36250: f64, t36253: f64, t36256: f64, t36259: f64, t36262: f64, t36266: f64, t36269: f64, t36276: f64, t36279: f64) -> f64 {
    let t37922 = 0.45351183609335988442e-1_f64 * t36236;
    let t37923 = 0.19055119163586549766e-2_f64 * t36238;
    let t37924 = 0.16006300097412701803e-1_f64 * t36240;
    let t37934 = 0.21437009059034868486e-2_f64 * t36273;
    let t37937 = 0.85748036236139473944e-3_f64 * t36283;
    let t37938 = 0.34299214494455789578e-1_f64 * t36286;
    let t37939 = t37922 - t37923 - t37924 + 0.62896184579208304137e-2_f64 * t36243 - 0.18868855373762491241e-2_f64 * t36246 - 0.37737710747524982482e-2_f64 * t36250 - t36253 / 12.0_f64 + t36256 / 64.0_f64 + t36259 / 48.0_f64 + t36262 / 96.0_f64 + 0.22921875e-1_f64 * t36266 + 0.1528125e-1_f64 * t36269 + t37934 + 0.42874018118069736972e-2_f64 * t36276 + 0.25724410870841842184e-2_f64 * t36279 + t37937 - t37938;
    t37939
}
