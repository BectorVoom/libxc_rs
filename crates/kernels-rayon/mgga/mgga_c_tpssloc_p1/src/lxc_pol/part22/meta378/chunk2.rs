//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1637/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1637(t17157: f64, t4510: f64, t17161: f64, t13798: f64, t17152: f64, t10236: f64, t5392: f64) -> (f64, f64, f64, f64) {
    let t17854 = t4510 * t17157;
    let t17857 = t4510 * t17161;
    let t17860 = t13798 * t17152;
    let t17863 = t10236 * t5392;
    (t17854, t17857, t17860, t17863)
}
