//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 603/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk603(t1611: f64, t2347: f64, t240: f64, t4535: f64, t555: f64, t6604: f64, t8186: f64, t8188: f64, t8191: f64, t8287: f64, t8432: f64, t8436: f64, t8455: f64) -> f64 {
    let t8459 = t8186 - t8188 + t8191 - t8287 + t240 * (-t1611 * t8455 - 2.0_f64 * t2347 * t6604 + 2.0_f64 * t4535 * t8436 + t555 * t8432 - t8186 + t8188 - t8191 + t8287);
    t8459
}
