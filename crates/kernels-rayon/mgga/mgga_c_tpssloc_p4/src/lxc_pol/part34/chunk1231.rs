//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1231/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1231(t105437: f64, t105441: f64, t17090: f64, t21034: f64, t26700: f64, t29056: f64, t29060: f64, t29091: f64, t4147: f64, t4268: f64, t5658: f64, t7087: f64, t7830: f64, t85060: f64, t86991: f64) -> f64 {
    let t108378 = -3.0_f64 * t26700 * t5658 + 6.0_f64 * t17090 * t7830 - t7087 * t21034 - 18.0_f64 * t4147 * t29091 - 3.0_f64 * t4147 * t29056 + 0.9869604401089358619e-1_f64 * t105437 - 0.38381794893125283518e0_f64 * t86991 + 6.0_f64 * t4268 * t29060 - 18.0_f64 * t4268 * t29091 - 0.49348022005446793095e-1_f64 * t105441 - t85060;
    t108378
}
