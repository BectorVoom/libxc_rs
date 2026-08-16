//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1072/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1072(t1218: f64, t1232: f64, t2134: f64, t32425: f64, t32429: f64, t32433: f64, t32436: f64, t32441: f64, t32445: f64, t32448: f64, t488: f64, t7316: f64, t7326: f64, t8875: f64) -> f64 {
    let t32451 = t32425 - 0.40372756094140390856e-3_f64 * t7316 * t8875 - 0.40372756094140390856e-3_f64 * t2134 * t32429 + 0.40372756094140390856e-3_f64 * t7326 * t32433 + t32436 * t488 / 1536.0_f64 + t32441 * t1218 / 1536.0_f64 + t32445 - t32448 * t1232 / 2304.0_f64;
    t32451
}
