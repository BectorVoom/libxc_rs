//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1159/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1159<F: Float>(t43101: F, t43102: F, t43106: F, t43111: F, t43115: F, t43119: F, t43122: F, t43125: F, t43127: F, t43131: F, t47661: F, t43134: F, t43137: F, t43139: F, t43143: F, t43146: F, t43147: F, t43148: F, t43152: F, t43154: F, t43156: F, t43157: F) -> (F, F) {
    let t47663 = t43101 - F::cast_from(0.30762104920568897135e-1_f64) * t43102 - t47661 - t43106 + t43111 + t43115 - t43119 + t43122 - t43125 + F::cast_from(0.32043859292259267849e-3_f64) * t43127 + t43131;
    let t47668 = -F::cast_from(0.15381052460284448567e-1_f64) * t43134 - F::cast_from(0.76905262301422242837e-2_f64) * t43137 + F::cast_from(0.42725145723012357132e-3_f64) * t43139 - t43143 + t43146 + t43147 - t43148 - t43152 + F::cast_from(0.23071578690426672851e-1_f64) * t43154 + t43156 + t43157;
    (t47663, t47668)
}
