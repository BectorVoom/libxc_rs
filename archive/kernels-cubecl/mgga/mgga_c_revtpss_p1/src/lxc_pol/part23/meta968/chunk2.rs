//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3268/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3268<F: Float>(t162: F, t85950: F, t85968: F, t187: F, t48297: F, t48304: F, t48306: F, t47093: F, t39989: F, t47084: F, t47086: F, t47088: F, t47092: F, t47096: F, t47098: F, t48300: F, t48303: F, t85928: F, t85930: F, t85932: F) -> (F, F, F, F, F, F, F) {
    let t85970 = (t85950 + t85968) * t162;
    let t85972 = F::cast_from(0.19751673498613801407e-1_f64) * t85970 * t187;
    let t85973 = F::cast_from(0.30762056574649219972e4_f64) * t48297;
    let t85974 = F::cast_from(0.48796115851357829289e-1_f64) * t48304;
    let t85975 = F::cast_from(0.14447919941302971323e1_f64) * t48306;
    let t85976 = F::cast_from(0.10389515463408878255e3_f64) * t47093;
    let t85977 = -t85928 + t85930 - t85932 + t85972 - t85973 - t47084 - t48300 + t48303 + t85974 + t85975 - t39989 - t47086 + t47088 + t47092 + t85976 - t47096 - t47098;
    (t85970, t85972, t85973, t85974, t85975, t85976, t85977)
}
