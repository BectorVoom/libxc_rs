//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1066/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1066<F: Float>(t36236: F, t36238: F, t36240: F, t36273: F, t36283: F, t36286: F, t36243: F, t36246: F, t36250: F, t36253: F, t36256: F, t36259: F, t36262: F, t36266: F, t36269: F, t36276: F, t36279: F) -> (F,) {
    let t37922 = 0.45351183609335988442e-1 * t36236;
    let t37923 = 0.19055119163586549766e-2 * t36238;
    let t37924 = 0.16006300097412701803e-1 * t36240;
    let t37934 = 0.21437009059034868486e-2 * t36273;
    let t37937 = 0.85748036236139473944e-3 * t36283;
    let t37938 = 0.34299214494455789578e-1 * t36286;
    let t37939 = t37922 - t37923 - t37924 + 0.62896184579208304137e-2 * t36243 - 0.18868855373762491241e-2 * t36246 - 0.37737710747524982482e-2 * t36250 - t36253 / 12.0 + t36256 / 64.0 + t36259 / 48.0 + t36262 / 96.0 + 0.22921875e-1 * t36266 + 0.1528125e-1 * t36269 + t37934 + 0.42874018118069736972e-2 * t36276 + 0.25724410870841842184e-2 * t36279 + t37937 - t37938;
    (t37939,)
}
