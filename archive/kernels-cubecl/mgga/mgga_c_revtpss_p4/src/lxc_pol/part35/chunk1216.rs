//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1216/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1216<F: Float>(t103114: F, t103122: F, t103130: F, t103158: F, t103161: F, t110340: F, t110344: F, t110346: F, t110355: F, t1558: F, t1579: F, t231: F, t25317: F, t27199: F, t28394: F, t30337: F, t30379: F, t30392: F, t6049: F, t6071: F, t7070: F, t7071: F, t7076: F, t8006: F, t99191: F) -> F {
    let t115551 = F::cast_from(0.26020884564615598386e1_f64) * t7070 * t7071 * t30379 * t1579 + F::cast_from(0.13010442282307799193e1_f64) * t7070 * t7076 * t30379 * t1558 * t231 - F::cast_from(0.78062653693846795158e1_f64) * t7070 * t25317 * t8006 * t6071 - F::cast_from(0.26020884564615598386e1_f64) * t27199 * t30392 + F::cast_from(0.39512695097613069591e1_f64) * t28394 * t6049 - F::cast_from(0.28912093960683998208e-1_f64) * t103114 + F::cast_from(0.21684070470512998656e-1_f64) * t110340 + F::cast_from(0.68549505033305214441e-2_f64) * t103122 + F::cast_from(0.72280234901709995519e-3_f64) * t103130 - F::cast_from(0.86736281882051994623e-1_f64) * t110344 - F::cast_from(0.38554277296572111609e-1_f64) * t110346 - F::cast_from(0.58544643236296698113e-1_f64) * t110355 - F::cast_from(0.52041769129231196772e1_f64) * t99191 * t30337 + F::cast_from(0.19514881078765566038e-2_f64) * t103158 + F::cast_from(0.34697458558045176417e-2_f64) * t103161;
    t115551
}
