//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1061/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1061(t2363: f64, t88: f64, t89: f64, t776: f64, t828: f64, t868: f64) -> (f64, f64, f64, f64) {
    let t12739 = t88 * t2363;
    let t12823 = t89 * t2363;
    let t13229 = t828 * t776;
    let t13487 = t776 * t868;
    (t12739, t12823, t13229, t13487)
}
