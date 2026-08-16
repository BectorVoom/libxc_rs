//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 852/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk852(t36700: f64, t44777: f64, t549: f64, t44712: f64, t739: f64, t1991: f64, t590: f64, t43389: f64, t5241: f64, t5640: f64, t43400: f64, t43403: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45183 = 0.47667319935800568892e0_f64 * t36700 * t549 * t44777;
    let t45184 = t739 * t44712;
    let t45187 = 0.1022478025437886658e1_f64 * t1991 * t45184 * t590;
    let t45188 = 0.14570311862489884877e2_f64 * t43389;
    let t45192 = 0.15337170381568299871e1_f64 * t5640 * t5241 * t44712 * t590;
    let t45193 = 0.30674340763136599742e1_f64 * t43400;
    let t45194 = 0.20705180015117204825e2_f64 * t43403;
    (t45183, t45187, t45188, t45192, t45193, t45194)
}
