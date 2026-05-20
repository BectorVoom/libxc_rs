//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2015/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2015<F: Float>(t99066: F, t99069: F, t99073: F, t99077: F, t93004: F, t93010: F, t93016: F, t95678: F, t95680: F, t95684: F, t99063: F, t99071: F, t99075: F) -> F {
    let t103315 = F::cast_from(0.16006300097412701803e0_f64) * t99066;
    let t103316 = F::cast_from(0.11433071498151929859e-3_f64) * t99069;
    let t103318 = F::cast_from(0.2032800112371413129e-2_f64) * t99073;
    let t103320 = F::cast_from(0.10164000561857065645e-3_f64) * t99077;
    let t103321 = F::cast_from(0.11433071498151929859e-3_f64) * t93004 + t95678 - F::cast_from(0.57165357490759649295e-3_f64) * t93010 - t99063 / F::new(2.0) - t95680 - F::cast_from(0.36143185997963725434e-4_f64) * t93016 - t95684 - t103315 - t103316 + F::cast_from(0.34299214494455789578e-1_f64) * t99071 + t103318 - F::cast_from(0.85748036236139473944e-3_f64) * t99075 - t103320;
    t103321
}
