//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1159/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1159(t43101: f64, t43102: f64, t43106: f64, t43111: f64, t43115: f64, t43119: f64, t43122: f64, t43125: f64, t43127: f64, t43131: f64, t47661: f64, t43134: f64, t43137: f64, t43139: f64, t43143: f64, t43146: f64, t43147: f64, t43148: f64, t43152: f64, t43154: f64, t43156: f64, t43157: f64) -> (f64, f64) {
    let t47663 = t43101 - 0.30762104920568897135e-1_f64 * t43102 - t47661 - t43106 + t43111 + t43115 - t43119 + t43122 - t43125 + 0.32043859292259267849e-3_f64 * t43127 + t43131;
    let t47668 = -0.15381052460284448567e-1_f64 * t43134 - 0.76905262301422242837e-2_f64 * t43137 + 0.42725145723012357132e-3_f64 * t43139 - t43143 + t43146 + t43147 - t43148 - t43152 + 0.23071578690426672851e-1_f64 * t43154 + t43156 + t43157;
    (t47663, t47668)
}
