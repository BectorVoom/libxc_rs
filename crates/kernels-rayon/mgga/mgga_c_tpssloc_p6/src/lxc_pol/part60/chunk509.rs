//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 509/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk509(t6197: f64, t6237: f64, t466: f64, t1760: f64, t3598: f64, t491: f64, t6224: f64, t3612: f64, t1734: f64, t1751: f64, t1246: f64, t6218: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6238 = t6197 + t6237;
    let t6239 = t466 * t6238;
    let t6243 = t1760 * t1760;
    let t6244 = t3598 * t6243;
    let t6252 = t491 * t6224;
    let t6253 = t6252 * t3612;
    let t6256 = t1751 * t1734;
    let t6257 = t6256 * t1246;
    let t6260 = t491 * t6218;
    (t6238, t6239, t6243, t6244, t6252, t6253, t6256, t6257, t6260)
}
