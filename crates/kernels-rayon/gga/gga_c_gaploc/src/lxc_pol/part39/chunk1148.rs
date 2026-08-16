//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1148/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1148(t47243: f64, t6066: f64, t6111: f64, t10914: f64, t10915: f64, t41448: f64, t44134: f64, t44138: f64, t44142: f64, t44144: f64, t44145: f64, t44148: f64, t44149: f64, t44150: f64, t44151: f64) -> f64 {
    let t47549 = t6111 * t6066 * t47243;
    let t47552 = t10914 * t10915 * t47243;
    let t47555 = 0.31952438294933958064e0_f64 * t41448;
    let t47556 = t44134 + 0.42900587942220512003e1_f64 * t47549 - 0.21450293971110256001e1_f64 * t47552 + t44138 + t44142 + t44144 - 0.10725146985555128001e1_f64 * t44145 + t47555 - t44148 + t44149 + t44150 + t44151;
    t47556
}
