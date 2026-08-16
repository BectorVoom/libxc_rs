//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 714/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk714(t1949: f64, t968: f64, t1920: f64, t225: f64, t6688: f64) -> (f64, f64, f64) {
    let t6781 = t968 * t1949;
    let t6783 = 0.27415567780803773942e-2_f64 * t1920 * t6781;
    let t6784 = t6688 * t225;
    (t6781, t6783, t6784)
}
