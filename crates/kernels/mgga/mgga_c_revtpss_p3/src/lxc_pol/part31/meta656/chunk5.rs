//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2210/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2210<F: Float>(t22267: F, t25997: F, t22255: F, t7264: F, t22259: F, t22276: F, t7271: F, t22281: F, t26024: F, t6876: F, t22289: F, t102498: F, t98169: F, t98174: F, t98181: F, t98186: F, t98188: F) -> F {
    let t108566 = t25997 * t22267;
    let t108568 = t7264 * t22255;
    let t108570 = t25997 * t22259;
    let t108572 = t7271 * t22276;
    let t108574 = t7271 * t22281;
    let t108576 = t26024 * t6876;
    let t108578 = t7271 * t22289;
    let t108580 = -t102498 - t98169 + F::cast_from(0.54208002996571016775e-3_f64) * t98174 - t98181 - F::cast_from(0.25410001404642664113e-4_f64) * t108566 - F::cast_from(0.42874018118069736972e-3_f64) * t108568 - F::cast_from(0.25410001404642664113e-4_f64) * t108570 - F::cast_from(0.51448821741683684367e-1_f64) * t108572 + F::cast_from(0.17149607247227894789e-1_f64) * t108574 + F::cast_from(0.20007875121765877254e-2_f64) * t108576 + F::cast_from(0.85748036236139473945e-2_f64) * t108578 + t98186 - t98188;
    t108580
}
